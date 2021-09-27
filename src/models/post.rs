use crate::schemas::post::posts;

use serde::{Serialize, Deserialize};
use async_graphql::ID;
use async_graphql::*;

/// # Entity to read from the database
#[derive(Queryable)]
pub struct PostEntity {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub post_type: String,
    pub content: String,
    pub published: bool,
}

/// # Entity for creating a record in the database
#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPostEntity<'a> {
    pub author: &'a i32,
    pub title: &'a str,
    pub post_type: &'a str,
    pub content: &'a str,
    pub published: &'a bool,
}

/// # Structure of the output data GraphQL
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: ID,
    pub author: ID,
    pub title: String,
    pub post_type: String,
    pub content: String,
    pub published: bool,
}

/// # Input data structure GraphQL
#[derive(InputObject)]
pub struct PostInput {
    pub author: ID,
    pub title: String,
    pub post_type: String,
    pub content: String,
    pub published: bool,
}

/// # Methods for displaying fields
#[Object]
impl Post {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn author(&self) -> &ID {
        &self.author
    }

    async fn title(&self) -> &String {
        &self.title
    }

    async fn post_type(&self) -> &String {
        &self.post_type
    }

    async fn content(&self) -> &String {
        &self.content
    }

    async fn published(&self) -> &bool {
        &self.published
    }
}
