
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


#[cfg(test)]
mod tests {

    use super::*;
    use crate::schema::posts::dsl::*;
    use crate::models::Post;

    #[test]
    fn test_query() {
        let connection = establish_connection();
        let results = posts.filter(published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("------------\n");
            println!("{}", post.body);
        }
    }
}
