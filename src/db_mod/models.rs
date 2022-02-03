use diesel::{Queryable, Insertable};
use serde::Deserialize;
use super::schema::discord_users;

#[derive(Queryable,Debug)]
pub struct DiscordUsers{
	pub id:i32,
	pub uname:String,
	pub discord_id:String,
}
#[derive(Insertable,Deserialize)]
#[table_name="discord_users"]
pub struct NewUser{
	pub uname:String,
	pub discord_id:String,
}
impl NewUser{
	pub fn new(name:&str,d_id:&str)->Self{
		Self{uname:name.into(),discord_id:d_id.into()}
	}
}
#[derive(Deserialize)]
pub struct CheckUser{
	pub discord_id:String,
}