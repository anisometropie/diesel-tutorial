use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::*; // import aliases, so that we have posts instead of posts::table and published instead of posts::published

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}