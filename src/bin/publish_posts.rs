extern crate testing_diesel;
extern crate diesel;

use self::diesel::prelude::*;
use self::testing_diesel::*;
use self::models::Post;
use std::env::args;
use std::time::SystemTime;

fn main() {
    use testing_diesel::schema::posts::dsl::{posts, published, publish_at};

    let id = args().nth(1).expect("publish_posts requires a post id").parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    if id > 0 {
        let post = diesel::update(posts.find(id))
        .set((published.eq(true),
              publish_at.eq(SystemTime::now())
            ))
        .get_result::<Post>(&connection)
        .expect(&format!("unable to find post with id {}", id));
        println!("Published post {}", post.title);
    }
    else {
        let query = diesel::update(posts)
            .set((published.eq(true), 
                  publish_at.eq(SystemTime::now())
                )
            );
        let updated_posts = query.get_results::<Post>(&connection)
            .expect(&format!("failed publishing all posts"));
        println!("Published {} posts", updated_posts.len());
        //println!("{}", diesel::debug_query::<diesel::pg::Pg, _>(&query));
    }
}