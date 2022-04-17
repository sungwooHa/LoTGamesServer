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
    pub verify_email: bool,
    pub profile_image: String,
    pub wallet_address: String,
}

impl ResponseUser {
    pub fn get_response_user_from_userdb(user_db: &User) -> ResponseUser {
        ResponseUser {
            user_id: user_db.userID.clone().unwrap_or_default(),
            nickname: user_db.nickname.clone().unwrap_or_default(),
            verify_email: {
                if let Some(verify) = user_db.verifyEmail {
                    if verify == 1_u8 {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            },
            profile_image: user_db.profileImage.clone().unwrap_or_default(),
            wallet_address: user_db.walletAddress.clone().unwrap_or_default(),
        }
    }
}

#[derive(FromForm, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUuidVerifyEmailHash {
    #[form(field = "uuid")]
    pub uuid: i64,

    #[form(field = "emailHash")]
    pub verify_email_hash: String,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct UserAddress {
    #[form(field = "walletAddress")]
    pub wallet_address: String,
}
