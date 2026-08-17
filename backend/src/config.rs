use std::str::FromStr;

pub struct Config {
  pub server_port: u16,
  pub username: String,
  pub password: String,
  pub db_url: String,
}

impl Config {
  pub fn new() -> Self {
    dotenvy::dotenv().ok();

    let server_port =
      Self::get_env::<u16>("SERVER_PORT");
    let username =
      Self::get_env::<String>("DB_USERNAME");
    let password =
      Self::get_env::<String>("DB_PASSWORD");
    let db_url =
      Self::get_env::<String>("DB_URL");

    Config {
      server_port,
      username,
      password,
      db_url,
    }
  }

  //
  // 環境変数を取得する為のジェネリック関数
  //
  fn get_env<T>(key: &str) -> T
  where
    T: FromStr,
  {
    std::env
            ::var(key)
            .unwrap_or_else(|_| { panic!("Environment variable {} is not set", key) })
            .parse()
            .unwrap_or_else(|_| {
                panic!("Environment variable {} is not a valid {}", key, std::any::type_name::<T>())
            })
  }
}
