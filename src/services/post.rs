use crate::schemas::post;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods, QueryResult};
use crate::models::post::{NewPostEntity, PostEntity};

pub fn get_all(conn: &PgConnection) -> Vec<PostEntity> {
    post::posts::dsl::posts.load::<PostEntity>(conn).expect("Error loading posts.")
}

pub fn get(id: i32, conn: &PgConnection) -> QueryResult<PostEntity> {
    post::posts::table.find(id).get_result::<PostEntity>(conn)
}

pub fn create(new: NewPostEntity, conn: &PgConnection) -> PostEntity {
    diesel::insert_into(post::posts::table)
        .values(new)
        .get_result::<PostEntity>(conn)
        .expect("Error saving new post")
}

pub fn update(post: PostEntity, conn: &PgConnection) -> PostEntity {
    diesel::update(post::posts::table.find(post.id))
        .set((
            post::posts::title.eq(post.title),
            post::posts::content.eq(post.content),
            post::posts::published.eq(post.published),
        ))
        .get_result::<PostEntity>(conn).expect("Error update post.")
}

// pub fn delete(id: i32, conn: &PgConnection) {
//     diesel::delete(posts.filter(id))
//         .execute(conn)
//         .expect("Error deleting posts");
// }
