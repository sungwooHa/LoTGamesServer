use diesel::MysqlConnection;
use diesel::{self, prelude::*};

use crate::db::model::user::UserToken;
use crate::db::schema::tbl_user_token::dsl::*;

pub fn get_user_by_uuid(conn: &MysqlConnection, _uuid: &i64) -> QueryResult<UserToken> {
    tbl_user_token
        .limit(1)
        .filter(uuid.eq(_uuid))
        .get_result::<UserToken>(&*conn)
}

pub fn update_user_token(conn: &MysqlConnection, user_token: &UserToken) -> QueryResult<usize> {
    diesel::update(tbl_user_token.find(user_token.uuid))
        .set(user_token)
        .execute(&*conn)
}
