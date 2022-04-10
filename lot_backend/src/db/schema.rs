table! {
    tbl_user (uuid) {
        uuid -> Bigint,
        userID -> Nullable<Varchar>,
        userPW -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        exceptArena -> Nullable<Integer>,
        regLastLoginDate -> Nullable<Datetime>,
        regDate -> Nullable<Datetime>,
        regIP -> Nullable<Varchar>,
        walletAddress -> Nullable<Varchar>,
        verifyEmailHash -> Nullable<Varchar>,
        verifyEmail -> Nullable<Unsigned<Tinyint>>,
        txHash -> Nullable<Varchar>,
        profileImage -> Nullable<Varchar>,
    }
}
