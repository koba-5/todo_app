use std::fmt::Display;

pub fn info<T: Display>(output: T) {
    println!(
        "[{}] {}",
        time::OffsetDateTime::now_utc(),
        output.to_string()
    )
}
