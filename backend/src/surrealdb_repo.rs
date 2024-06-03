use models::GeneralUser;
use std::sync::Arc;
// use surrealdb::{sql::Value, Datastore, Error, Session};
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

#[derive(Clone, Debug)]
pub struct Database {
    pub client: Surreal<Client>,
    pub name_space: String,
    pub db_name: String,
}

#[derive(Debug, Deserialize)]
struct Count {
    count: i64,
}

impl Database {
    pub async fn init() -> Result<Self, Error> {
        let client = Surreal::new::<Ws>("127.0.0.1:8001").await?;
        client
            .signin(Root {
                // TODO: do not hard code.
                username: "user",
                password: "password",
            })
            .await?;
        // Name Space is like a level above a database
        client
            .use_ns("surreal")
            .use_db("quiz_app")
            .await
            .expect("Unable to connect to database");
        Ok(Database {
            client,
            name_space: "surreal".into(),
            db_name: String::from("quiz_app"),
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

        let count: Option<Count> = response.take(0)?;
        dbg!(&count);
        match count {
            Some(count) => Ok(count.count),
            None => Ok(0),
        }
    }
}
