
#[allow(non_snake_case)]
diesel::table!{
    tbl_user(uuid) {
        uuid-> BigInt,
        userID -> Varchar,
        userPW -> Varchar,
        nickname -> Varchar,
        exceptArena -> Integer,
        regLastLoginDate -> Datetime,
        regDate -> Datetime,
        regIP -> VarChar,
	    walletAddress -> VarChar,
        verifyEmailHash -> VarChar,
        verifyEmail -> Tinyint,
        txHash -> Varchar,
    }
}