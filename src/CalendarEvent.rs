// Model for event object

// future reference for supporting other calendar types?
// docs.rs/epochs/latest/epochs/
// from imports
use chrono::prelude::*;

// TODO: Not useful in single-user application
// pending  : no response
// accepted : responded, affirmative
// denied   : responded, negative
// unsure   : responded, unsure
#[derive(Clone)]
pub enum EventStatus {
    Pending,
    Accepted,
    Denied,
    Unsure
}

// todo: constrain length of vectors? strings?
#[derive(Clone)]
pub struct Event {
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

// ALL PLACEHOLDERS, PLS IGNORE
impl Event {
    pub fn create_calendar_event(&mut self) {
        println!("{:?}",self.name);
    }

    pub fn view_calendar_event(&mut self) {
        println!("{:?}",self.name);
        println!("{:?}", self.start_time.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    pub fn edit_calendar_event(&mut self) {
        println!("{:?}",self.name);
    }

    pub fn delete_calendar_event(&mut self) {
        println!("{:?}",self.name);
    }
}
