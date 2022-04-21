use crate::constants::{message_constants, url_constants};
use crate::db::connection::Conn;
use crate::db::models::User;
use crate::db::query;
use crate::model::response::{Response, ResponseWithStatus};
use crate::model::user_dto::{self, InsertableUser, VerifyUser};
use crate::util::mail_system::MailSubjectType;
use crate::util::{hash_generator, mail_system};

use rocket::http::Status;
use time::Duration;

pub fn get_user_by_wallet(conn: &Conn, wallet_address: &String) -> ResponseWithStatus {
    if let Ok(user) = query::get_user_by_wallet_address(&conn, &wallet_address) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value(user_dto::ResponseUser::get_response_user_from_userdb(
                    &user,
                ))
                .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_NOT_FOUND),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn verify_user_by_uuid_with_email_hash(
    conn: &Conn,
    uuid: &i64,
    verify_email_hash: &String,
) -> ResponseWithStatus {
    //Get User Info

    if hash_generator::is_expired_hash(&verify_email_hash){
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_EXPIRED_HASH),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let user = match query::get_user_by_uuid_with_email_hash(&conn, &uuid, &verify_email_hash) {
        Ok(mut user) => {
            user.verifyEmail = Some(1);
            user
        }
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND),
                    data: serde_json::to_value("").unwrap(),
                },
            };
        }
    };

    if query::update_user(&conn, &user).is_ok() {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CANT_VERIFY),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn sign_in_without_verify(conn: &Conn, verify_user: &VerifyUser) -> ResponseWithStatus {
    if query::get_user_by_email(&conn, &verify_user.email).is_ok() {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_DUPLICATED_EMAIL),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let verify_email_hash = hash_generator::generate_expired_hash(
        &verify_user.email, Duration::hours(1));

    if query::insert_user(&conn, {
        &User {
            userID: Some(verify_user.email.clone()),
            walletAddress: Some(verify_user.wallet_address.clone()),
            verifyEmailHash: Some(verify_email_hash.clone()),
            ..Default::default()
        }
    })
    .is_err()
    {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_INSERT),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let uuid =
        if let Ok(user) = query::get_user_by_wallet_address(&conn, &verify_user.wallet_address) {
            user.uuid.to_string()
        } else {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_FAIL_INSERT),
                    data: serde_json::to_value("").unwrap(),
                },
            };
        };

    let mail_contents = format!(
        "{}/users/verify?uuid={}&emailHash={}",
        url_constants::LOT_SRV_URL,
        uuid,
        &verify_email_hash
    );

    if mail_system::send_mail(
        &verify_user.email,
        &MailSubjectType::MailVerify,
        &mail_contents,
    )
    .is_ok()
    {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_SEND_MAIL),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

pub fn sign_in_final(conn: &Conn, insertable_user: &InsertableUser) -> ResponseWithStatus {
    let user = match query::get_user_by_wallet_address(&conn, &insertable_user.wallet_address) {
        Ok(mut user) => {
            user.userPW = Some(hash_generator::generate_expired_hash(
                &insertable_user.wallet_address,
                Duration::days(1)
            ));
            user.nickname = Some(insertable_user.nickname.clone());
            user.txHash = Some(insertable_user.txhash.clone());
            user.profileImage = Some(insertable_user.profile_image.clone());
            user
        }
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND),
                    data: serde_json::to_value("").unwrap(),
                },
            };
        }
    };

    if query::update_user(&conn, &user).is_err() {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_UPDATE),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    if mail_system::send_mail(
        &user.userID.unwrap(),
        &MailSubjectType::UserPassword,
        &user.userPW.unwrap(),
    )
    .is_ok()
    {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_SEND_MAIL),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}
