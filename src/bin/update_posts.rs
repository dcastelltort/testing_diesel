extern crate testing_diesel;
extern crate diesel;

use self::diesel::prelude::*;

use self::testing_diesel::*;
use self::testing_diesel::models::*;
use std::io::{stdin , Read};
use std::env::args;

fn main() {

    use testing_diesel::schema::posts::dsl::{posts};

    let id = args().nth(1).expect("update_posts requires a post id").parse::<i32>().expect("Invalid ID");
    let connection = establish_connection();

    let mut post = posts.find(id).get_result::<Post>(&connection).expect(&format!("No post for id {}", id));

    println!("Current title is: {}", post.title);
    println!("Please type in replacement");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let new_size = title.len() - 1;
    title.truncate(new_size); //remove \n
    post.title = title;
    
    println!("Current body is :");
    println!("{}", post.body);
    println!("\nOk! Let's write a new one (Press {} when finished)\n", EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();
    post.body = body;
    post.published = false;
    //diesel::update(posts.find(id)).set(&post).execute(&connection).expect("failed to save updated post");
    post.save_changes::<Post>(&connection).expect("failed to save updated post"); // much simpler and equivalent to the above
    println!("\nSaved post {} with id {} as draft, publish it again", post.title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
