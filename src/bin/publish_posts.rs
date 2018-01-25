extern crate testing_diesel;
extern crate diesel;

use self::diesel::prelude::*;
use self::testing_diesel::*;
use self::models::Post;
use std::env::args;

fn main() {
    use testing_diesel::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_posts requires a post id").parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("unable to find post with id {}", id));
    println!("Published post {}", post.title);
}