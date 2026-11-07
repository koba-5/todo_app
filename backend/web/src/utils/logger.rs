use std::fmt::Display;

pub fn info<T: Display>(output: T) {
    println!(
        "[{}][INFO] {}",
        time::OffsetDateTime::now_utc(),
        output.to_string()
    )
}

pub fn warn<T: Display>(output: T) {
    println!(
        "[{}][WARN] {}",
        time::OffsetDateTime::now_utc(),
        output.to_string()
    )
}

pub fn error<T: Display>(output: T) {
    eprintln!(
        "[{}][ERROR] {}",
        time::OffsetDateTime::now_utc(),
        output.to_string()
    )
}
