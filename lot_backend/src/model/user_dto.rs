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
pub struct RequestWalletAddress {
    #[form(field = "walletAddress")]
    pub wallet_address: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenWithCount {
    pub token: String,
    pub count: u32,
}
