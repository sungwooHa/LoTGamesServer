table! {
    tbl_auth (seq) {
        seq -> Bigint,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        walletAddress -> Nullable<Varchar>,
        verifyEmailHash -> Nullable<Varchar>,
        verifyEmail -> Nullable<Bool>,
        txHash -> Nullable<Varchar>,
        regDate -> Nullable<Datetime>,
    }
}

table! {
    tbl_user (uuid) {
        uuid -> Bigint,
        nickname -> Nullable<Varchar>,
        exceptArena -> Nullable<Integer>,
        profileImage -> Nullable<Varchar>,
        regLastLoginDate -> Nullable<Datetime>,
        regDate -> Nullable<Datetime>,
    }
}

table! {
    tbl_user_token (uuid) {
        uuid -> Nullable<Bigint>,
        token -> Nullable<Text>,
        regDate -> Nullable<Datetime>,
    }
}

allow_tables_to_appear_in_same_query!(tbl_auth, tbl_user, tbl_user_token,);
