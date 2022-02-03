use super::{
    error_handler::{ApiError, CheckResponse},
    models::{CheckUser, DiscordUsers, NewUser},
    schema::discord_users,
    PoolConn,
};
use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use discord_users::dsl::*;

pub async fn add_db(pg_conn: PoolConn, data: web::Json<NewUser>) -> Result<(), ApiError> {
    let user = NewUser::new(&data.uname, &data.discord_id);
    let res = diesel::insert_into(discord_users::table)
        .values(&user)
        .on_conflict(discord_id)
        .do_nothing()
        .execute(&pg_conn);
    match res {
        Err(err) => Err(ApiError::DbError(err)),
        Ok(_val) => Ok(()),
    }
    // .expect("Error adding data to Db");
    // .get_result(&pg_conn)
    // .unwrap()
}

// pub async fn disp_db(pg_conn:PoolConn)->Result<(),ApiError>{
// 	let result=discord_users::dsl::discord_users
// 				.load::<DiscordUsers>(&pg_conn)
// 				.expect("Error loading data from Db");
// 	for res in result{
// 		println!("{:?}",res);
// 	}
// 	Ok(())
// }

pub async fn check_db(
    pg_conn: PoolConn,
    data: web::Json<CheckUser>,
) -> Result<CheckResponse, ApiError> {
    let d_id: String = format!("{}", data.discord_id);
    let result = discord_users::dsl::discord_users
        .filter(discord_users::dsl::discord_id.eq(d_id))
        .load::<DiscordUsers>(&pg_conn);
    match result {
        Ok(val) => {
            if val.len() == 0 {
                Ok(CheckResponse::CheckFlag(true))
            } else {
                Ok(CheckResponse::CheckFlag(false))
            }
        }
        Err(err) => Err(ApiError::DbError(err)),
    }
}
