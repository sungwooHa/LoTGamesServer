use diesel::MysqlConnection;
use diesel::{self, prelude::*};

use crate::db::model::user::User;
use crate::db::schema::tbl_user::dsl::*;

pub fn insert_user(conn: &MysqlConnection, user: &User) -> QueryResult<usize> {
    diesel::insert_into(tbl_user).values(user).execute(&*conn)
}

pub fn get_user_by_uuid(conn: &MysqlConnection, _uuid: &i64) -> QueryResult<User> {
    tbl_user
        .limit(1)
        .filter(uuid.eq(_uuid))
        .get_result::<User>(&*conn)
}
