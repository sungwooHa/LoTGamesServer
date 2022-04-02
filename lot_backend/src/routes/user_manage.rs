

use crate::db::models::User;
use crate::db::models::InsertableUser;
use crate::db::schema;
use crate::db::connection::Conn;
use crate::db::query;

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
        userID : "user_id".to_string(),
        userPW : "user_pw".to_string(),
        nickname : "nickname".to_string(),
        exceptArena : 100,
        regLastLoginDate : NaiveDate::from_ymd(2022, 4, 3).and_hms(1,1,1),
        regDate : NaiveDate::from_ymd(2022, 4, 3).and_hms(1,1,1),
        regIP : "reg_ip".to_string(),
        walletAddress : "wallet_address".to_string(),
        verifyEmailHash : "verify_email_hash".to_string(),
        verifyEmail : 1,
        txHash : "tx_hash".to_string(),
    };
    Json(user)
}

#[get("/db_test")]
pub fn db_test(conn: Conn) -> Result<Json<Vec<User>>, Status> {
    let result = query::show_users(&conn)
        .map(|user| Json(user))
        .map_err(|err| error_status(err));

        for user in query::show_users(&conn).unwrap(){
            println!("{:?}", user);
        }

    result
}

fn error_status(err : Error) ->Status{
    match err {
        Error::NotFound => Status::Ok,
        _=>Status::InternalServerError,
    }
}