// TODO: If project grows, Add SessionStorage to different SurrealDB Instance
use crate::configuration::DatabaseSettings;
use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::cookie::time::Duration;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use models::GeneralUser;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::{Id, Thing};
use surrealdb::{Error, Surreal};

#[derive(Clone, Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

#[derive(Debug, Deserialize)]
struct GeneralUserCount {
    count: i64,
}

impl Database {
    /// Creating Database connection from configuration in YAML files.
    pub async fn from_config(config: DatabaseSettings) -> Result<Self, Error> {
        let address: String = format!("{}:{}", config.host, config.port);
        println!("{:?}", &address);
        let client = Surreal::new::<Ws>(&address).await?;
        println!("Signing in");
        client
            .signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await?;
        println!("Getting namespace and database");
        // Name Space is like a level above a database
        client
            .use_ns(&config.namespace)
            .use_db(&config.name)
            .await
            .expect("Unable to connect to database");

        println!("Returning the goods");
        Ok(Database {
            client,
            name_space: config.namespace,
            db_name: config.name,
        })
    }

    pub async fn get_all_general_users(&self) -> Option<Vec<GeneralUser>> {
        let result = self.client.select("general_user").await;
        match result {
            Ok(all_gen_users) => Some(all_gen_users),
            Err(_) => None,
        }
    }
    pub async fn add_general_user(&self, new_general_user: GeneralUser) -> Option<GeneralUser> {
        let created_gen_user: Result<Option<GeneralUser>, Error> = self
            .client
            .create(("general_user", new_general_user.uuid.clone()))
            .content(new_general_user)
            .await;

        match created_gen_user {
            Ok(this) => this,
            Err(_) => None,
        }
    }
    pub async fn count_users(&self, username: &str) -> surrealdb::Result<i64> {
        let qry = r#"SELECT count() FROM type::table($table)
        WHERE username = $username"#;
        let mut response: surrealdb::Response = self
            .client
            .query(qry)
            .bind(("table", "general_user"))
            .bind(("username", username))
            .await?;

        let count: Option<GeneralUserCount> = response.take(0)?;
        dbg!(&count);
        match count {
            Some(count) => Ok(count.count),
            None => Ok(0),
        }
    }
}

// -- Below is for Session Store --

type SessionState = HashMap<String, String>;

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct SessionToken {
    id: Thing,
    token: String,
    expiry: surrealdb::sql::Datetime,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub struct UpdatedSessionToken {
    token: Option<String>,
    expiry: Option<surrealdb::sql::Datetime>,
}

/// Generates Random SessionKey for creating session tokens
fn generate_session_key() -> anyhow::Result<SessionKey> {
    let key: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    let session_key: SessionKey = key.try_into().context("Invalid Session Key")?;
    Ok(session_key)
}

/// Adding time to current time for setting expiry of cookie
fn generate_time_stamp(dur: &Duration) -> anyhow::Result<DateTime<Utc>> {
    let duration_mili: i64 = dur
        .whole_milliseconds()
        .try_into()
        .context("Invalid duration")?;
    let time_stamp: DateTime<Utc> =
        DateTime::<Utc>::from_timestamp_millis(Utc::now().timestamp_millis() + duration_mili)
            .context("Issue creating timestamp")?;

    Ok(time_stamp)
}

impl SessionStore for Database {
    #[tracing::instrument(
        name = "Loading Session Token"
        skip_all
    )]
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let cache_key = session_key.as_ref();

        let token_info: Thing = Thing {
            tb: "sessions".to_string(),
            id: Id::String(String::from(cache_key)),
        };

        // Getting value from database
        let session_token_res: surrealdb::Result<Option<SessionToken>> =
            self.client.select(token_info.clone()).await;

        // Extracting value or error
        let session_token_opt: Option<SessionToken> = if let Ok(res) = session_token_res {
            res
        } else {
            tracing::warn!("Error getting result from database");
            return Err(LoadError::Other(anyhow::anyhow!("Database Error")));
        };

        let surreal_token: SessionToken = match session_token_opt {
            Some(token) => token,
            None => return Ok(None),
        };

        // Check if expired
        if surreal_token.expiry.timestamp_millis() < Utc::now().timestamp_millis() {
            let _: Option<SessionToken> = self
                .client
                .delete(token_info)
                .await
                .expect("Deleting token in database failed");
            return Ok(None);
        }

        Ok(serde_json::from_str(&surreal_token.token)
            .context("Failed to deserialize session state")
            .map_err(LoadError::Deserialization)?)
    }
    /// `SessionKey` is a `String` wrapper.
    async fn save(
        &self,
        session_state: SessionState,
        time_to_live: &Duration,
    ) -> Result<SessionKey, SaveError> {
        // Try serialize session data to store in database
        let data: String = serde_json::to_string(&session_state)
            .context("Failed to serialize session state")
            .map_err(SaveError::Serialization)?;

        let session_key: SessionKey = generate_session_key().map_err(SaveError::Other)?;

        let expiry: DateTime<Utc> = generate_time_stamp(time_to_live).map_err(SaveError::Other)?;

        let _: Vec<SessionToken> = self
            .client
            .create("sessions")
            .content(SessionToken {
                id: Thing {
                    tb: "sessions".to_owned(),
                    id: Id::String(session_key.as_ref().to_owned()),
                },
                token: data,
                expiry: expiry.into(),
            })
            .await
            .context("Failed to create record in database")
            .map_err(SaveError::Other)?;

        Ok(session_key)
    }
    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        time_to_live: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let data: String = serde_json::to_string(&session_state)
            .context("Failed to serialize session state")
            .map_err(UpdateError::Serialization)?;

        // These are instruction on what to update, and where it lives.
        let token_info: Thing = Thing {
            tb: "sessions".to_string(),
            id: Id::String(session_key.as_ref().to_owned()),
        };

        let updated_expiry: DateTime<Utc> =
            generate_time_stamp(time_to_live).map_err(UpdateError::Other)?;

        let updated_token: UpdatedSessionToken = UpdatedSessionToken {
            token: Some(data),
            expiry: Some(updated_expiry.into()),
        };

        // Perform update or return error
        let _: Option<UpdatedSessionToken> = self
            .client
            .update(token_info)
            .merge(updated_token)
            .await
            .context("Failed to create record in database")
            .map_err(UpdateError::Other)?;

        Ok(session_key)
    }

    async fn update_ttl(
        &self,
        session_key: &SessionKey,
        time_to_live: &Duration,
    ) -> Result<(), anyhow::Error> {
        let token_info: Thing = Thing {
            tb: "sessions".to_string(),
            id: Id::String(session_key.as_ref().to_owned()),
        };

        if !time_to_live.is_positive() {
            // If duration is non-positive we force remove cookie.
            let _: Option<SessionToken> = self
                .client
                .delete(token_info)
                .await
                .context("Deleting token in database failed")?;
        } else {
            // Else, if time is positive, we update
            let updated_expiry: DateTime<Utc> =
                generate_time_stamp(time_to_live).map_err(UpdateError::Other)?;
            let updated_token: UpdatedSessionToken = UpdatedSessionToken {
                token: None,
                expiry: Some(updated_expiry.into()),
            };
            let _: Option<UpdatedSessionToken> = self
                .client
                .update(token_info)
                .merge(updated_token)
                .await
                .context("Failed to create record in database")?;
        }
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let token_info: Thing = Thing {
            tb: "sessions".to_string(),
            id: Id::String(session_key.as_ref().to_owned()),
        };

        let _: Option<SessionToken> = self
            .client
            .delete(token_info)
            .await
            .context("Deleting token in database failed")?;

        Ok(())
    }
}
