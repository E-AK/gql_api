pub mod post;

use crate::PgPool;
use crate::schemas::post::Query;

use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use std::sync::Arc;


pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    let arc_pool = Arc::new(pool);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(arc_pool)
        .finish()
}