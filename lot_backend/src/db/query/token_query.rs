use diesel::{self, prelude::*};
use diesel::MysqlConnection;

use crate::db::model::user::UserToken;
use crate::db::schema::tbl_user_token::dsl::*;

pub fn get_user_by_uuid(conn: &MysqlConnection, _uuid: &i64) -> QueryResult<UserToken> {
    tbl_user_token
        .limit(1)
        .filter(uuid.eq(_uuid))
        .get_result::<UserToken>(&*conn)
}
