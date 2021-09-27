use api::{create_connection_pool, configure_service, run_migrations};
use api::schemas::create_schema_with_context;

use actix_web::{HttpServer, App};
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let port = if args.len() < 2 { "8080" } else { &args[1] };

    let pool = create_connection_pool();
    run_migrations(&pool);
    let schema = create_schema_with_context(pool);

    HttpServer::new(move || App::new().configure(configure_service).data(schema.clone()))
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}