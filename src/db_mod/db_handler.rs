use super::models::{NewUser,DiscordUsers};
use super::schema::discord_users;
use super::error_handler::ApiError;
use diesel::{pg::PgConnection, Connection,RunQueryDsl};
use dotenv::dotenv;
use std::env;
use actix_web::{web};

fn create_conn()->PgConnection{
	dotenv().ok();
	let db_url=env::var("DATABASE_URL").unwrap();
	PgConnection::establish(&db_url).unwrap()
}

pub fn add_db(pg_conn:PgConnection,data:web::Json<NewUser>)->Result<(),ApiError>{
	let user=NewUser::new(&data.uname,&data.discord_id);
	diesel::insert_into(discord_users::table)	
	.values(&user)
	.execute(&pg_conn)
	.unwrap();
	Ok(())
	// .get_result(&pg_conn)
	// .unwrap()
}

pub fn disp_db(pg_conn:PgConnection)->Result<(),ApiError>{
	let result=discord_users::dsl::discord_users
				.load::<DiscordUsers>(&pg_conn)
				.unwrap();
	for res in result{
		println!("{:?}",res);
	}				
	Ok(())
}
