use crate::db::connection::Conn;
use crate::db::models::InsertableUser;
use crate::db::models::User;
use crate::db::models::VerifyUser;
use crate::db::query;
use crate::util::hash_generator;
use crate::util::mail_system;
use crate::util::mail_system::MailSubjectType;

use chrono::Utc;
use diesel;
use diesel::result::Error;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[get("/hello")]
pub fn hello() -> Json<User> {
    Json(User {
        uuid: 12345,
        userID: None,
        userPW: Some("user_pw".to_string()),
        nickname: Some("nickname".to_string()),
        exceptArena: Some(100),
        regLastLoginDate: Some(Utc::now().naive_utc()),
        regDate: Some(Utc::now().naive_utc()),
        regIP: Some("reg_ip".to_string()),
        walletAddress: Some("wallet_견ㅅaddress".to_string()),
        verifyEmailHash: Some("verify_email_hash".to_string()),
        verifyEmail: Some(1),
        txHash: Some("tx_hash".to_string()),
        profileImage: Some("profile Image".to_string()),
    })
}

#[get("/db")]
pub fn db(conn: Conn) -> Result<Json<Vec<User>>, Status> {
    query::show_users(&conn).map(Json).map_err(error_status)
}

#[get("/users/address/<wallet_address>")]
pub fn get_user_by_wallet(conn: Conn, wallet_address: String) -> Result<Json<User>, Status> {
    query::get_user_by_wallet_address(&conn, &wallet_address)
        .map(Json)
        .map_err(error_status)
}

#[get("/users/verify/<uuid>/<verify_email_hash>")]
pub fn verify_user_by_uuid_with_eamil_hash(
    conn: Conn,
    uuid: i64,
    verify_email_hash: String,
) -> Result<Json<usize>, Status> {
    //Get User info.
    let user = match query::get_user_by_uuid_with_email_hash(&conn, &uuid, &verify_email_hash) {
        Ok(mut user) => {
            user.verifyEmail = Some(1);
            user
        }
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    };

    query::update_user(&conn, &user)
        .map(Json)
        .map_err(error_status)
}

#[post("/users", format = "application/json", data = "<verify_user>")]
pub fn sign_in_no_verify(conn: Conn, verify_user: Json<VerifyUser>) -> Status {
    let verify_email_hash = hash_generator::generate_hash_with_time(&verify_user.email);

    println!(
        "hash : {} / input data : {:?}",
        verify_email_hash, verify_user.email
    );

    let insert_res = query::insert_user(&conn, {
        &User {
            userID: Some(verify_user.email.clone()),
            walletAddress: Some(verify_user.wallet_address.clone()),
            verifyEmailHash: Some(verify_email_hash.clone()),
            ..Default::default()
        }
    });

    if insert_res.is_err() {
        return Status::InternalServerError;
    }

    match mail_system::send_mail(
        &verify_user.email,
        &MailSubjectType::MailVerify,
        &verify_email_hash,
    ) {
        Ok(_) => Status::Ok,
        Err(err) => {
            println!("Mail Error : {:?}", err);
            Status::InternalServerError
        }
    }
}

#[put("/users", format = "application/json", data = "<insertable_user>")]
pub fn sign_in_final(conn: Conn, insertable_user: Json<InsertableUser>) -> Status {
    let user = match query::get_user_by_wallet_address(&conn, &insertable_user.wallet_address) {
        Ok(mut user) => {
            user.userPW = Some(hash_generator::generate_hash_with_time(
                &insertable_user.wallet_address,
            ));
            user.nickname = Some(insertable_user.nickname.clone());
            user.txHash = Some(insertable_user.txhash.clone());
            user.profileImage = Some(insertable_user.profile_image.clone());
            user
        }
        Err(_) => {
            return Status::InternalServerError;
        }
    };

    if query::update_user(&conn, &user).is_err() {
        return Status::InternalServerError;
    }

    match mail_system::send_mail(
        &user.userID.unwrap(),
        &MailSubjectType::UserPassword,
        &user.userPW.unwrap(),
    ) {
        Ok(_) => Status::Ok,
        Err(err) => {
            println!("Mail Error : {:?}", err);
            Status::InternalServerError
        }
    }
}

fn error_status(err: Error) -> Status {
    match err {
        Error::NotFound => Status::Ok,
        _ => Status::InternalServerError,
    }
}
