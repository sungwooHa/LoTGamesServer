

use crate::db::models::User;
use crate::db::models::InsertableUser;
use crate::db::schema;
use crate::db::connection::Conn;
use crate::db::query;

use chrono::Utc;
use diesel::{self, prelude::*};
use diesel::result::Error;
use rocket_contrib::json::Json;
use rocket::http::Status;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[get("/hello")]
pub fn hello() -> Json<User>{
    let user = User{
        uuid : 12345,
        userID : None,
        userPW : Some("user_pw".to_string()),
        nickname : Some("nickname".to_string()),
        exceptArena : Some(100),
        regLastLoginDate : Some(Utc::now().naive_utc()),
        regDate : Some(Utc::now().naive_utc()),
        regIP : Some("reg_ip".to_string()),
        walletAddress : Some("wallet_address".to_string()),
        verifyEmailHash : Some("verify_email_hash".to_string()),
        verifyEmail : Some(1),
        txHash : Some("tx_hash".to_string())
    };
    Json(user)
}

#[get("/db")]
pub fn db(conn: Conn) -> Result<Json<Vec<User>>, Status> {
    query::show_users(&conn)
        .map(|user| Json(user))
        .map_err(|err| error_status(err))
}

#[get("/users/address/<wallet_address>")]
pub fn get_user_by_wallet(conn: Conn, wallet_address : String) -> Result<Json<Vec<User>>, Status> {

    diesel::insert_into(tbl_user)
    .values(&vec![
        (verifyEmailHash.eq(Some(verify_email_hash))),
        ]
    )
    .execute(&*conn)

    query::get_user_by_wallet_address(&conn, wallet_address)
        .map(|user| Json(user))
        .map_err(|err| error_status(err))
}

fn error_status(err : Error) ->Status{
    match err {
        Error::NotFound => Status::Ok,
        _=>Status::InternalServerError,
    }
}