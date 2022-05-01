use serde_json::Value;

use crate::db::model::user::{User, UserAuthInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseUser {
    pub email: String,
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
            email: user_auth_info.email.clone().unwrap_or_default(),
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
