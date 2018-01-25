extern crate testing_diesel;
extern crate diesel;

use self::diesel::prelude::*;
use self::testing_diesel::*;
use std::env::args;

fn main() {
    use testing_diesel::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against post title");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");
    println!("Delete {} posts", num_deleted);
}