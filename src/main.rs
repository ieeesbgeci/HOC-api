#[macro_use] extern crate diesel;
#[path="./db_mod/db_handler.rs"] pub mod db_handler;
#[path="./db_mod/models.rs"] pub mod models;
pub mod error_handler;
pub mod schema;
use dotenv::dotenv;
use std::env;
use actix_web::{web,App,HttpServer,Result};
use actix_cors::Cors;
use serde::{Serialize};
use self::models::NewUser;
#[actix_web::main]
async fn main()->std::io::Result<()>{
	dotenv().ok();
	// let white_list=env::var("ORIGINS").unwrap();
	let port=env::var("PORT").unwrap();
	let host=env::var("HOSt").unwrap();
	let ip_port=format!("{}:{}",host,port);	
	println!("server running on : {}",ip_port);
	HttpServer::new(| | { 
					//test-env cors :)
					//use white_list env variable to white_list origins in production
                    let cors = Cors::permissive();
                                //  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]);
                    App::new()
                        .wrap(cors)
						.route("/add_data",web::post().to(add_data)) 
					})
					.bind(ip_port)?
					.run()
					.await
}

async fn add_data(_res:web::Json<NewUser>)->Result<web::Json::<Response>>{
	let invite_link=std::env::var("INVITE_LINK").unwrap();
	Ok(web::Json(Response::new(invite_link)))
}

#[derive(Serialize)]
pub struct Response{
	result:String,
}
impl Response{
	pub fn new(result:String)->Self{
		Self{result}
	}
}
