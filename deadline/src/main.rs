use chrono::prelude::*;
use chrono::{DateTime, Local, TimeZone};

struct Event {
    name: String, 
    date: DateTime<Local>, 
}

trait Deadline{
    fn is_passed(&self)->bool;
}

impl Deadline for Event{
    fn is_passed(&self)->bool{
        self.date > Local::now()
    }
}

fn main() {
    let diwali: Event= Event{
        name: String::from("Diwali"),
        date:  chrono::Local.ymd(2019, 3, 17).and_hms(16, 43, 0),
    }; 

    if !diwali.is_passed() {
        println!("Visit your family next year"); 
    }
    else {
        println!("Happy Diwali!");
    }
    
}
