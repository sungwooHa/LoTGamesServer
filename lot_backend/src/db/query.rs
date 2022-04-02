#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::db::models::User;
use crate::db::schema::tbl_user::dsl::*;


//SELECT * from user limit 5
pub fn show_users(conn: &MysqlConnection) -> QueryResult<Vec<User>> {
    tbl_user.limit(5).load::<User>(&*conn)
}