#[macro_use]
extern crate diesel;
#[path = "./db_mod/db_handler.rs"]
pub mod db_handler;
pub mod error_handler;
#[path = "./db_mod/models.rs"]
pub mod models;
pub mod schema;
use self::models::{CheckUser, NewUser};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Result};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use dotenv::dotenv;
use error_handler::CheckResponse;
use r2d2;
use serde::Serialize;
use std::env;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type PoolConn = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // let white_list=env::var("ORIGINS").unwrap();
    let db_pool = init_dbpool().await;
    let port = env::var("PORT").expect("Error parsing Port Var");
    let host = env::var("HOST").expect("Error parsing HOST Var");
    let ip_port = format!("{}:{}", host, port);
    println!("server running on : {}", ip_port);
    HttpServer::new(move || {
        //test-env cors :)
        //use white_list env variable to white_list origins in production
        let cors = Cors::permissive();
        //  .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]);
        App::new()
            .wrap(cors)
            .data(db_pool.clone())
            .route("/add_data", web::post().to(add_data))
            // .route("/disp_data",web::get().to(disp_data))
            .route("/check_data", web::post().to(check_data))
    })
    .bind(ip_port)
    .expect("Error binding to Port")
    .run()
    .await
}

async fn add_data(
    db_pool: web::Data<DbPool>,
    res: web::Json<NewUser>,
) -> Result<web::Json<Response>> {
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let res = db_handler::add_db(db_conn, res).await;
    match res {
        Err(error_handler::ApiError::DbError(err)) => {
            println!("[DB_ERROR]:\n{}", err);
            Ok(web::Json(Response::new("Error adding data".into())))
        }
        _ => {
            let invite_link =
                std::env::var("INVITE_LINK").expect("Error parsing INVITE_LINK variable");
            Ok(web::Json(Response::new(invite_link)))
        }
    }
}

// async fn disp_data(db_pool:web::Data<DbPool>)->Result<web::Json::<Response>>{
// 	let db_conn=db_pool.get().expect("Error getting Dbconnector");
// 	db_handler::disp_db(db_conn).await.unwrap_or(());
// 	let invite_link=std::env::var("INVITE_LINK").expect("Error parsing INVITE_LINK Var");
// 	Ok(web::Json(Response::new(invite_link)))
// }

async fn check_data(
    db_pool: web::Data<DbPool>,
    res: web::Json<CheckUser>,
) -> Result<web::Json<Response>> {
    let db_conn = db_pool.get().expect("Error creating Dbconnector");
    let resp = db_handler::check_db(db_conn, res).await;
    match resp {
        Ok(CheckResponse::CheckFlag(true)) => Ok(web::Json(Response::new("Id found".into()))),
        Ok(CheckResponse::CheckFlag(false)) => Ok(web::Json(Response::new("Id not found".into()))),
        Err(err) => {
            println!("[Error]:\n{:?}", err);
            Ok(web::Json(Response::new("Error processsing request".into())))
        }
    }
    // let invite_link=std::env::var("INVITE_LINK").expect("Error parsing INVITE_LINK Var");
}
#[derive(Serialize)]
pub struct Response {
    result: String,
}
impl Response {
    pub fn new(result: String) -> Self {
        Self { result }
    }
}
pub async fn init_dbpool() -> DbPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();
    let conn_manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder()
        .build(conn_manager)
        .expect("Error building Dbconnector")
}
