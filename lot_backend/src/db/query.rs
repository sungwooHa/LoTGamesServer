#![allow(proc_macro_derive_resolution_fallback)]

use diesel::query_builder::IncompleteInsertStatement;
use diesel::{self, prelude::*};
use crate::db::models::User;
use crate::db::schema::tbl_user::dsl::*;
use rocket_contrib::json::Json;

//SELECT * from user limit 5
pub fn show_users(conn: &MysqlConnection) -> QueryResult<Vec<User>> {

    // tbl_user.load(&*conn).map_err(|err|-> String {
    //     println!("Error querying user: {:?}", err);
    //     "Error querying user from the database".into()
    // }).map(Json)

    tbl_user.limit(5).load::<User>(&*conn)
}

pub fn by_wallet_address(conn : &MysqlConnection, _wallet_address : String) -> QueryResult<Vec<User>> {
    tbl_user.limit(1).filter(walletAddress.eq(_wallet_address)).load::<User>(&*conn)
}

pub fn get_user_by_wallet_address(conn : &MysqlConnection, _wallet_address : String) -> QueryResult<Vec<User>> {
    tbl_user.limit(1).filter(walletAddress.eq(_wallet_address)).load::<User>(&*conn)
}

pub fn get_user_by_uuid_with_email_hash(conn: &MysqlConnection, _uuid : i64, _verify_email_hash : String) -> QueryResult<Vec<User>> {
    tbl_user.limit(1).filter(uuid.eq(_uuid)).filter(verifyEmailHash.eq(_verify_email_hash)).load::<User>(&*conn)
}
