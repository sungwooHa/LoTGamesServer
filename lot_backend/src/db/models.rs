use chrono::NaiveDateTime;
use crate::db::schema::tbl_user;

#[allow(non_snake_case)]
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[table_name = "tbl_user"]
pub struct User {
    pub uuid : i64,
    pub userID : Option<String>,
    pub userPW : Option<String>,
    pub nickname : Option<String>,
    pub exceptArena : Option<i32>,
    pub regLastLoginDate : Option<NaiveDateTime>,
    pub regDate : Option<NaiveDateTime>,
    pub regIP : Option<String>,
    pub walletAddress : Option<String>,
    pub verifyEmailHash : Option<String>,
    pub verifyEmail : Option<u8>,
    pub txHash : Option<String>,
}

#[allow(non_snake_case)]
#[derive(/*Queryable, AsChangeset,*/ Serialize, Deserialize)]
pub struct InsertableUser {
    pub uuid: i64,
    pub walletAddress : String,
    pub verifyEmailHash : String,
    pub verifyEmail : i8,
    pub txHash : String,
}