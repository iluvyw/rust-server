use serde::{Serialize};

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

//Inserting post

use super::schema::posts;

#[derive(Insertable, Serialize)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}