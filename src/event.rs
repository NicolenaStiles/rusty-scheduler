// Model for event object

// future reference for supporting other calendar types?
// docs.rs/epochs/latest/epochs/

use chrono::prelude::*;

// let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11);

// todo: constrain length of vectors? strings?
pub struct Event {
    pub name : String,
    pub user : String,
    pub start_time : DateTime<Utc>, // in UTC
    pub end_time : DateTime<Utc>, // in UTC
    pub is_public : bool,
    pub location : String,  // optional
    pub status : String, // TODO: this should be an enum of pending/accepted/deined/maybe
    pub is_recurring : bool, // TODO: tether together if true
    pub invite_list : Vec<String>
}

impl Event {
    pub fn create_event(&mut self) {
        println!("{:?}",self.initial_state);
    }

    pub fn view_event(&mut self) {
        println!("{:?}",self.initial_state);
    }

    pub fn edit_event(&mut self) {
        println!("{:?}",self.current_state);
    }

    pub fn delete_event(&mut self) {
        println!("{:?}",self.solution);
    }
}
