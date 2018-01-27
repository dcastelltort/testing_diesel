extern crate testing_diesel;
extern crate diesel;

extern crate serde;
extern crate serde_json;

use self::diesel::prelude::*;
use self::testing_diesel::*;
use self::models::{NewPost, Post};
use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    use testing_diesel::schema::posts::dsl::{posts};

    let filename = args().nth(1).expect("import_posts requires valid json input file").parse::<String>().expect("Invalid filename");
    let connection = establish_connection();

    let mut f = File::open(filename).expect("import file not found");

    let mut json = String::new();
    f.read_to_string(&mut json)
        .expect("something went wrong reading the json file");

    let new_posts = serde_json::from_str::<Vec<NewPost>>(&json).expect("failed parsing json content");

    let inserted_posts = diesel::insert_into(posts).values(&new_posts).get_results::<Post>(&connection).expect("failed inserting new posts");

    let mut ids = vec!();
    for p in &inserted_posts {
        ids.push(&p.id);
    }

    println!("{}", format!("Imported posts ids (don't forget to publish them): {:?}", ids));
}