extern crate diesel;
extern crate psql1;

use self::diesel::prelude::*;
use self::psql1::*;
use std::env::args;

fn main() {
    use psql1::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();

    let number_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", number_deleted);
}
