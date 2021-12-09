// Entry point for rusty scheduler

// reference:
// https://rust-lang-nursery.github.io/rust-cookbook/datetime.html
// https://docs.rs/chrono/latest/chrono/

// from imports
use chrono::prelude::*;

// from local/other files
mod CalendarEvent;

/*
#[derive(Clone)]
pub struct CalendarEvent {
    pub name : String,
    pub user : String,
    pub start_time : DateTime<Utc>, // in UTC
    pub end_time : DateTime<Utc>, // in UTC
    pub is_public : bool,
    pub location : String,  // optional
    pub status : EventStatus,
    pub is_recurring : bool, // TODO: tether together if true
    pub invite_list : Vec<String>
}
*/

fn main() {
    println!("Hello, scheduler!");

    // test case: game night with friends
    // these can all be static, actually
//    let name = String::from("Apex Legends Game Night");
//    let user = String::from("sn00z");
//    let start_time = Utc.ymd(2021, 12, 10).and_hms(20, 30, 8);
//    let end_time = Utc.ymd(2021, 12, 10).and_hms(22, 30, 8);
//    let is_public : bool = true;
//    let location = String::from("online - plague drs");
//    let status = CalendarEvent::EventStatus::Pending;
//    let is_recurring : bool = false;
    // this one is a bit long, so I'll leave it seperate
    let friends = vec!["sn00z".to_string(), "heiressevna".to_string(), "ShitBitchBear".to_string()];

    let mut game_night_event = CalendarEvent::Event { name : String::from("Apex Legends Game Night"),
                                                      user : String::from("sn00z"),
                                                      start_time : Utc.ymd(2021, 12, 10).and_hms(20, 30, 8),
                                                      end_time : Utc.ymd(2021, 12, 10).and_hms(22, 30, 8),
                                                      is_public : true,
                                                      location : String::from("online - plague drs"),
                                                      status : CalendarEvent::EventStatus::Pending,
                                                      is_recurring : false,
                                                      invite_list : friends
    };

    // testing getting current datetime info
    //let dt_now : DateTime<Utc> = Utc::now();

    // setting datetime info
    //let birthdate = Utc.ymd(1995, 5, 22).and_hms(12, 50, 8); // `2014-07-08T09:10:11Z`

    // printing/unravelling format
    //println!("{:?}", dt_now.format("%Y-%m-%d %H:%M:%S").to_string());
    //println!("{:?}", birthdate.format("%Y-%m-%d %H:%M:%S").to_string());

    game_night_event.view_calendar_event();
}
