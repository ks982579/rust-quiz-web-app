use models::GeneralUser;
use std::sync::Arc;
// use surrealdb::{sql::Value, Datastore, Error, Session};
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::configuration::DatabaseSettings;

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
