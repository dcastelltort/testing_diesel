use super::schema::posts;
use std::time::SystemTime;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub publish_at: SystemTime,
    pub visit_count: i32
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str
}