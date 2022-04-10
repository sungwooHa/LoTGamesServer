#![allow(proc_macro_derive_resolution_fallback)]

use diesel::query_builder::IncompleteInsertStatement;
use diesel::{self, prelude::*};
use diesel::update;

use crate::db::models::User;
use crate::db::schema::tbl_user::dsl::*;
use diesel::result::Error;
use rocket_contrib::json::Json;

//SELECT * from user limit 5
pub fn show_users(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
    tbl_user.limit(5).load::<User>(&*conn)
}

pub fn get_user_by_wallet_address(conn : &MysqlConnection, _wallet_address : &String) -> QueryResult<User> {
    match tbl_user.limit(1).filter(walletAddress.eq(_wallet_address)).load::<User>(&*conn) {
        Ok(arr_user) => {
            match arr_user.get(0) {
                Some(user) => Ok(user.clone()),
                None => {return Err(Error::NotFound);},
            }
        }
        Err(error) => {return Err(error);},
    }
}

pub fn get_user_by_uuid_with_email_hash(conn: &MysqlConnection, _uuid : &i64, _verify_email_hash : &String) -> QueryResult<User> {
    tbl_user.limit(1).filter(uuid.eq(_uuid)).filter(verifyEmailHash.eq(_verify_email_hash)).get_result::<User>(&*conn)
}

pub fn insert_user(conn : &MysqlConnection, user : &User) -> QueryResult<usize>{
    diesel::insert_into(tbl_user).values(user).execute(&*conn)
}

pub fn update_user(conn : &MysqlConnection, user : &User) -> QueryResult<usize>{
    diesel::update(tbl_user.find(user.uuid))
        .set(user)
        .execute(&*conn)
}