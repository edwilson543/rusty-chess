use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::establish_connection;

    #[test]
    fn can_establish_connection() {
        establish_connection();
    }
}
