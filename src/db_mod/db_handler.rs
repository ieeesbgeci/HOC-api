use super::{models::{NewUser,DiscordUsers,CheckUser},PoolConn,schema::discord_users,error_handler::ApiError};
use diesel::{QueryDsl,RunQueryDsl,ExpressionMethods};
use actix_web::web;

pub async fn add_db(pg_conn:PoolConn,data:web::Json<NewUser>)->Result<(),ApiError>{
	let user=NewUser::new(&data.uname,&data.discord_id);
	diesel::insert_into(discord_users::table)	
	.values(&user)
	.on_conflict_do_nothing()
	.execute(&pg_conn)
	.unwrap();
	Ok(())
	// .get_result(&pg_conn)
	// .unwrap()
}

pub async fn disp_db(pg_conn:PoolConn)->Result<(),ApiError>{
	let result=discord_users::dsl::discord_users
				.load::<DiscordUsers>(&pg_conn)
				.unwrap();
	for res in result{
		println!("{:?}",res);
	}				
	Ok(())
}

pub async fn check_db(pg_conn:PoolConn,data:web::Json<CheckUser>)->Result<(),ApiError>{
	let d_id:String=format!("{}",data.discord_id);
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
