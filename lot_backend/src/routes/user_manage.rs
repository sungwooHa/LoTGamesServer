

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
use rocket::response::status;
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
        walletAddress : Some("wallet_견ㅅaddress".to_string()),
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
    query::get_user_by_wallet_address(&conn, wallet_address)
        .map(|user| Json(user))
        .map_err(|err| error_status(err))
}

#[get("/users/verify/<uuid>/<verify_email_hash>")]
pub fn verify_user_by_uuid_with_eamil_hash(conn : Conn, uuid : i64, verify_email_hash : String) -> Status{
    
    let result  = query::get_user_by_uuid_with_email_hash(&conn, uuid, verify_email_hash)
                            .map(|user| Json(user))
                            .map_err(|err| error_status(err));

    match result {
        Ok(val) => {
            //verify_email으ㄹ true로 바꿔야함.
            println!("{:?}", val);
            Status::Ok
        }
        Err(_) => Status::InternalServerError,
    }

    //없을 때는?
    //verify_eamil을 true로 바꿔주기
    //이미 있는 경우엔 어떻게 할까?

}

// #[post("users/<email>/<wallet_address>")]
// pub fn sign_in_no_verify(conn : Conn, email : String, wallet_address : String) -> Result<_, Status>{

//     //db insert,
//     //send mail.

//     //step1. verify_eamil_hash와 만들기
//     //step2. verify_eamil_hash와 wallet_address로 table insert.
//     //step3. mail 로 uuid와 vverify_eamil_hash와

//     let user_no_verify = diesel::insert_into(tbl_user)
//     .values(&vec![
//         (verifyEmailHash.eq(Some(verify_email_hash))),
//         ]
//     )
//     .execute(&*conn)?;

    
//     Ok< ,Status()>
// }

// #[put("users/<wallet_address>/<txhash>/<nickname>")]
// pub fn sign_in_final(conn : Conn, wallet_address : String, txhash:String, nickname : String) -> Result<_, Status>{

//     //wallet address로 찾
//     //txhash, nickname 맞춰줌
//     query::get::user::by_wallet_address(&conn, wallet_address)
//     .map(|user| Json(user))
//     .map_err(|err| error_status(err))
// }

fn error_status(err : Error) ->Status{
    match err {
        Error::NotFound => Status::Ok,
        _=>Status::InternalServerError,
    }
}