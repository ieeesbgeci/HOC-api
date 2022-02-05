use super::schema::discord_users;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Queryable, Debug)]
pub struct DiscordUsers {
    pub id: i32,
    pub uname: String,
    pub name:String,
    pub e_mail:String,
}
#[derive(Insertable, Deserialize)]
#[table_name = "discord_users"]
pub struct NewUser {
    pub uname: String,
    pub name:String,
    pub e_mail:String,
}
impl NewUser {
    pub fn new(uname: &str,name:&str,email:&str) -> Self {
        Self {
            uname: uname.into(),
            name:name.into(),
            e_mail:email.into(),
        }
    }
}
#[derive(Deserialize)]
pub struct CheckUser {
    pub uname: String,
}
