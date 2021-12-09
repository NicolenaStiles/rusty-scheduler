// Entry point for rusty scheduler

// reference:
// https://rust-lang-nursery.github.io/rust-cookbook/datetime.html
// https://docs.rs/chrono/latest/chrono/
use chrono::prelude::*;

fn main() {
    println!("Hello, scheduler!");

    // testing getting current datetime info
    let dt_now : DateTime<Utc> = Utc::now();

    // setting datetime info
    let birthdate = Utc.ymd(1995, 5, 22).and_hms(12, 50, 8); // `2014-07-08T09:10:11Z`

    // printing/unravelling format
    println!("{:?}", dt_now.format("%Y-%m-%d %H:%M:%S").to_string());
    println!("{:?}", birthdate.format("%Y-%m-%d %H:%M:%S").to_string());
}
