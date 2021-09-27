pub mod schemas;
pub mod models;
pub mod services;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[macro_use]
extern crate diesel_migrations;
extern crate strum;

use crate::schemas::AppSchema;

use actix_web::{HttpResponse, web};
use dotenv::dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;
use async_graphql::Context;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{Request, Response};


embed_migrations!();

pub fn run_migrations(pool: &PgPool) {
    let conn = pool.get().expect("Can't get DB connection");
    embedded_migrations::run(&conn).expect("Failed to run database migrations");
}

pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

pub async fn index(schema: web::Data<AppSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_connection_pool() -> PgPool {
    dotenv().ok();

    let user = env::var("POSTGRES_USER").expect("Not POSTGRES_USER ENV Var!");
    let pass = env::var("POSTGRES_PASSWORD").expect("Not POSTGRES_PASSWORD ENV Var!");
    let db = env::var("POSTGRES_DB").expect("Not POSTGRES_DB ENV Var!");

    let db_url = format!("postgres://{}:{}@db:5432/{}", user, pass, db);

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> Conn {
    ctx.data::<Arc<PgPool>>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}
