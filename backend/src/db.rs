use oracle::Connection;

pub struct Db {
  pub conn: Connection,
}

impl Db {
  pub fn new(
    username: String,
    password: String,
    db_url: String,
  ) -> Self {
    let conn = Connection::connect(
      &username, &password, &db_url,
    )
    .expect("Failed to connect to database");

    Db { conn }
  }
}
