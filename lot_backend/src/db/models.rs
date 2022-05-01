use crate::db::schema::{tbl_auth, tbl_user, tbl_user_token};
use chrono::NaiveDateTime;

#[allow(non_snake_case)]
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone, Insertable, Default)]
#[table_name = "tbl_auth"]
pub struct UserAuth{
    pub seq : i64,
    pub email : Option<String>,
    pub password : Option<String>,
    pub walletAddress : Option<String>,
    pub verifyEmailHash : Option<String>,
    pub verifyEmail : Option<bool>,
    pub txHash : Option<String>,
    pub regDate : Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone, Insertable, Default)]
#[table_name = "tbl_user"]
pub struct User{
    pub uuid : i64,
    pub nickname : Option<String>,
    pub exceptArena : Option<i32>,
    pub profileImage : Option<String>,
    pub regLastLoginDate : Option<NaiveDateTime>,
    pub regDate : Option<NaiveDateTime>,
}

#[allow(non_snake_case)]
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone, Insertable, Default)]
#[table_name = "tbl_user_token"]
pub struct UserToken{
    pub uuid : Option<i64>,
    pub token : Option<String>,
    pub regDate : Option<NaiveDateTime>,
}
