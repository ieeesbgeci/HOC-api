use super::models::{NewUser,DiscordUsers};
use super::schema::discord_users;
use super::error_handler::ApiError;
use diesel::{QueryDsl,pg::PgConnection,RunQueryDsl,ExpressionMethods};
use actix_web::web;

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

pub fn check_db(pg_conn:PgConnection,d_id:String)->Result<(),ApiError>{
	let result=discord_users::dsl::discord_users
				.filter(discord_users::dsl::discord_id.eq(d_id))
				.load::<DiscordUsers>(&pg_conn)
				.unwrap();
	if result.len()	==0{
		println!("Discord ID not found :)!");
	}		
	for res in result{
		println!("{:?}",res);
	}				
	Ok(())
}
