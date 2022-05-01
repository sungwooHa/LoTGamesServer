use crate::db::model::user::{User, UserAuthInfo};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyUser {
    pub email: String,
    pub wallet_address: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertableUser {
    pub wallet_address: String,
    pub txhash: String,
    pub nickname: String,
    pub profile_image: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseUser {
    pub user_id: String,
    pub nickname: String,
    pub verify_email: bool,
    pub verify_wallet: bool,
    pub profile_image: String,
}

impl ResponseUser {
    pub fn get_response_user_from_userdb(
        user_db: &User,
        user_auth_info: &UserAuthInfo,
    ) -> ResponseUser {
        ResponseUser {
            user_id: user_db.uuid.clone().to_string(),
            nickname: user_db.nickname.clone().unwrap_or_default(),
            verify_email: {
                if let Some(verify) = user_auth_info.verifyEmail {
                    verify
                } else {
                    false
                }
            },
            verify_wallet: user_auth_info.txHash.is_some(),
            profile_image: user_db.profileImage.clone().unwrap_or_default(),
        }
    }
}

#[derive(FromForm, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserUuidVerifyEmailHash {
    #[form(field = "uuid")]
    pub uuid: i64,

    #[form(field = "emailHash")]
    pub verify_email_hash: String,
}

#[derive(FromForm, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAddress {
    #[form(field = "walletAddress")]
    pub wallet_address: String,
}
