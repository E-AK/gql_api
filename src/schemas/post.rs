use crate::models::post::{PostEntity, Post, PostInput, NewPostEntity};
use crate::services::post::{get_all, get, create, update};
use crate::get_conn_from_ctx;

use async_graphql::{Context, Schema, EmptySubscription};
use async_graphql::*;


table! {
    posts (id) {
        id -> Integer,
        author -> Integer,
        title -> Varchar,
        post_type -> Varchar,
        content -> Text,
        published -> Bool,
    }
}

pub struct Query;
pub struct Mutation;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;


impl From<&PostEntity> for Post {
    /// ## The method for converting an entity to a GraphQL structure
    fn from(entity: &PostEntity) -> Self {
        Post {
            id: entity.id.into(),
            author: entity.author.into(),
            title: entity.title.clone(),
            post_type: entity.post_type.clone(),
            content: entity.content.clone(),
            published: entity.published.into()
        }
    }
}

#[Object]
impl Query {
    async fn get_posts(&self, ctx: &Context<'_>) -> Vec<Post> {
        get_all(&get_conn_from_ctx(ctx))
            .iter()
            .map(Post::from)
            .collect()
    }


    async fn get_post(&self, ctx: &Context<'_>, id: ID) -> Option<Post> {
        get(parse_id(id), &get_conn_from_ctx(ctx))
            .ok()
            .map(|p| Post::from(&p))
    }
}

#[Object]
impl Mutation {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        post: PostInput,
    ) -> async_graphql::static_assertions::_core::result::Result<Post, Error> {
        let new_post = NewPostEntity {
            author: &parse_id(post.author),
            title: &post.title,
            post_type: &post.post_type,
            content: &post.content,
            published: &post.published,
        };

        let new_post_entity = create(new_post, &get_conn_from_ctx(ctx));

        Ok(Post::from(&new_post_entity))
    }

    async fn update_post(
        &self,
        ctx: &Context<'_>,
        post: PostInput,
        id: ID
    ) -> async_graphql::static_assertions::_core::result::Result<Post, Error> {
        let new_post = PostEntity {
            id: parse_id(id),
            author: parse_id(post.author),
            title: post.title,
            post_type: post.post_type,
            content: post.content,
            published: post.published
        };

        let new_post_entity = update(new_post, &get_conn_from_ctx(ctx));

        Ok(Post::from(&new_post_entity))
    }
}


fn parse_id(id: ID) -> i32 {
    let id = id
        .to_string()
        .parse::<i32>()
        .expect("Can't get id from String");

    id
}