use diesel::MysqlConnection;
use diesel::{self, prelude::*};

use crate::db::model::user::UserAuthInfo;
use crate::db::schema::tbl_auth::dsl::*;

pub fn get_user_by_wallet_address(
    conn: &MysqlConnection,
    _wallet_address: &String,
) -> QueryResult<UserAuthInfo> {
    tbl_auth
        .limit(1)
        .filter(walletAddress.eq(_wallet_address))
        .get_result::<UserAuthInfo>(&*conn)
}

pub fn get_user_by_email(conn: &MysqlConnection, _email: &String) -> QueryResult<UserAuthInfo> {
    tbl_auth
        .limit(1)
        .filter(email.eq(_email))
        .get_result::<UserAuthInfo>(&*conn)
}

pub fn insert_user(conn: &MysqlConnection, user: &UserAuthInfo) -> QueryResult<usize> {
    diesel::insert_into(tbl_auth).values(user).execute(&*conn)
}

pub fn get_user_by_uuid_with_email_hash(
    conn: &MysqlConnection,
    _uuid: &i64,
    _verify_email_hash: &String,
) -> QueryResult<UserAuthInfo> {
    tbl_auth
        .limit(1)
        .filter(seq.eq(_uuid))
        .filter(verifyEmailHash.eq(_verify_email_hash))
        .get_result::<UserAuthInfo>(&*conn)
}

pub fn update_user(conn: &MysqlConnection, user: &UserAuthInfo) -> QueryResult<usize> {
    diesel::update(tbl_auth.find(user.seq))
        .set(user)
        .execute(&*conn)
}
