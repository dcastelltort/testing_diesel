extern crate testing_diesel;
extern crate diesel;

use self::testing_diesel::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use testing_diesel::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {

        //update visit count
        diesel::update(&post).set(visit_count.eq(visit_count + 1)).execute(&connection).expect(&format!("failed updating visit count on post id {}", post.id));

        //display it
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
        println!("----- debug output ---");
        println!("{:?}", post);
        println!("----------------------");
    }
}