use std::time::SystemTime;
use chrono::{ DateTime, Utc} ;


pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub mobilephone: String,
    pub creation_date: SystemTime,
    pub last_update: SystemTime,
    pub is_active: bool,
}

impl User {

    pub fn creation_date(&self) -> String {
        let datetime: DateTime<Utc> = self.creation_date.into();
        return format!("{}", datetime.format("%+"));
    }
    pub fn last_update(&self) -> String {
        let datetime: DateTime<Utc> = self.last_update.into();
        return format!("{}", datetime.format("%+"));
    }
}
