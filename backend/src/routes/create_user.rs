//! backend/src/routes/create_user.rs
//! Endpoint used for user creation given credentials.
use crate::surrealdb_repo::Database;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpRequest, HttpResponse};
use models::GeneralUser;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateUserPayload {
    name: String,
    username: String,
    password: String,
}

impl Into<GeneralUser> for CreateUserPayload {
    fn into(self) -> GeneralUser {
        let uuid: String = Uuid::new_v4()
            .hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string();
        GeneralUser::new(uuid, self.name, self.username, self.password)
    }
}

/// Takes in JSON with user information and stores in database.
/// If successful, returns 201 CREATED.
#[tracing::instrument(name = "Request to Create User")]
pub async fn create_user(
    req: HttpRequest,
    db: web::Data<Database>,
    user_info_pt: web::Json<CreateUserPayload>,
) -> HttpResponse {
    let user_data = user_info_pt.into_inner();

    // Is username unique?
    let users = db
        .count_users(&user_data.username)
        .await
        .expect("issue connecting");

    if users > 0 {
        return HttpResponse::BadRequest().finish();
    }

    let new_user_opt: Option<GeneralUser> = db.add_general_user(user_data.into()).await;
    let new_user = new_user_opt.unwrap();
    // println!("{user_info:?}");
    HttpResponse::Created()
        .content_type(ContentType::json())
        .json(new_user)
}
