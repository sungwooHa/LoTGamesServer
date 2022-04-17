use crate::db::models::User;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyUser {
    pub email: String,
    pub wallet_address: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertableUser {
    pub wallet_address: String,
    pub txhash: String,
    pub nickname: String,
    pub profile_image: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    pub user_id: String,
    pub nickname: String,
    pub verify_email: u8,
    pub profile_image: String,
}

impl ResponseUser {
    pub fn get_response_user_from_userdb(user_db : &User) -> ResponseUser {
        ResponseUser{
            user_id : user_db.userID.clone().unwrap_or_default(),
            nickname : user_db.nickname.clone().unwrap_or_default(),
            verify_email : user_db.verifyEmail.clone().unwrap_or_default(),
            profile_image : user_db.profileImage.clone().unwrap_or_default(),
        }
    }
}