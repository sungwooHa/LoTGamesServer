#![allow(proc_macro_derive_resolution_fallback)]

use diesel::{self, prelude::*};

use crate::db::models::User;
use crate::db::schema::tbl_user::dsl::*;


pub fn get_user_by_wallet_address(
    conn: &MysqlConnection,
    _wallet_address: &String,
) -> QueryResult<User> {
    tbl_user
        .limit(1)
        .filter(walletAddress.eq(_wallet_address))
        .get_result::<User>(&*conn)
}

pub fn get_user_by_email(
    conn: &MysqlConnection,
    _email : &String,
) -> QueryResult<User> {
    tbl_user
        .limit(1)
        .filter(userID.eq(_email))
        .get_result::<User>(&*conn)
}

pub fn get_user_by_uuid_with_email_hash(
    conn: &MysqlConnection,
    _uuid: &i64,
    _verify_email_hash: &String,
) -> QueryResult<User> {
    tbl_user
        .limit(1)
        .filter(uuid.eq(_uuid))
        .filter(verifyEmailHash.eq(_verify_email_hash))
        .get_result::<User>(&*conn)
}

pub fn insert_user(conn: &MysqlConnection, user: &User) -> QueryResult<usize> {
    diesel::insert_into(tbl_user).values(user).execute(&*conn)
}

pub fn update_user(conn: &MysqlConnection, user: &User) -> QueryResult<usize> {
    diesel::update(tbl_user.find(user.uuid))
        .set(user)
        .execute(&*conn)
}
