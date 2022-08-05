use actix_web::{
    get, 
    post, 
    // put,
    error::ResponseError,
    // web::Path,
    web::Json,
    // web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use derive_more::{Display};
use serde::{Deserialize, Serialize};

extern crate api;
extern crate diesel;

use self::api::*;
use self::models::*;
use self::diesel::prelude::*;

#[derive(Debug, Display)]
pub enum PostError {
    PostNotFound,
    PostUpdateFailure,
    PostCreationFailure,
    BadPostRequest
}

impl ResponseError for PostError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PostError::PostNotFound => StatusCode::NOT_FOUND,
            PostError::PostUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            PostError::PostCreationFailure => StatusCode::FAILED_DEPENDENCY,
            PostError::BadPostRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/post")]
pub async fn get_all_posts () -> Result<Json<Vec<Post>>, PostError> {
    use api::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .load::<Post>(&connection)
        .expect("Error loading posts");
    
    Ok(Json(results))
    // Err(PostError::PostNotFound);
}

#[derive(Deserialize, Serialize)]
pub struct SubmitPostRequest {
    title: String,
    body: String
}

#[post("/post")]
pub async fn submit_post (request: Json<SubmitPostRequest>) -> Result<Json<NewPost>, PostError> {
    use schema::posts;
    let conn = establish_connection();
    let new_post = NewPost {
        title: request.title.to_string(),
        body: request.body.to_string()
    };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&conn)
        .expect("Error saving new post");
    Ok(Json(new_post))
}