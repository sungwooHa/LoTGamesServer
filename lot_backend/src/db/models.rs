use chrono::NaiveDateTime;
use crate::db::schema::tbl_user;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, AsChangeset, Queryable, Debug)]
#[table_name = "tbl_user"]
pub struct User {
    pub uuid : i64,
    pub userID : String,
    pub userPW : String,
    pub nickname : String,
    pub exceptArena : i32,
    pub regLastLoginDate : NaiveDateTime,
    pub regDate : NaiveDateTime,
    pub regIP : String,
    pub walletAddress : String,
    pub verifyEmailHash : String,
    pub verifyEmail : i8,
    pub txHash : String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Insertable)]
#[table_name = "tbl_user"]
pub struct InsertableUser {
    pub uuid: i64,
    pub walletAddress : String,
    pub verifyEmailHash : String,
    pub verifyEmail : i8,
    pub txHash : String,
}