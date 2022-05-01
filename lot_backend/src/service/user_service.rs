use crate::constants::{message_constants, url_constants};
use crate::db::connection::Conn;
use crate::db::model::user;
use crate::db::query::*;
use crate::model::contract_dto::ContractUser;
use crate::model::response::{Response, ResponseWithStatus};
use crate::model::user_dto::{self, InsertableUser, VerifyUser};
use crate::util::mail_system::MailSubjectType;
use crate::util::{hash_generator, mail_system};

use dotenv::dotenv;
use rocket::http::Status;
use serde_json::json;
use std::env;
use time::Duration;

pub fn get_user_by_wallet(conn: &Conn, wallet_address: &String) -> ResponseWithStatus {
    let user_auth_info = match auth_query::get_user_by_wallet_address(&conn, &wallet_address) {
        Ok(user_auth_info) => user_auth_info,
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND_USER_AUTH_INFO),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    };

    if let Ok(user) = user_query::get_user_by_uuid(&conn, &user_auth_info.seq) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value(user_dto::ResponseUser::get_response_user_from_userdb(
                    &user,
                    &user_auth_info,
                ))
                .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_NOT_FOUND_USER),
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

    if hash_generator::is_expired_hash(&verify_email_hash) {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_EXPIRED_HASH),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let user_auth_info =
        match auth_query::get_user_by_uuid_with_email_hash(&conn, &uuid, &verify_email_hash) {
            Ok(mut user_auth_info) => {
                user_auth_info.verifyEmail = Some(true);
                user_auth_info
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

    if auth_query::update_user(&conn, &user_auth_info).is_ok() {
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

pub fn find_email_from_contract(verify_user: &VerifyUser) -> ResponseWithStatus {
    dotenv().ok();
    let lcd_address = env::var("LCD_ADDRESS").expect("LCD_ADDRESS must be set");
    let lot_contract = env::var("LOT_CONTRACT").expect("LOT_CONTRACT must be set");

    let encode_parsed = base64::encode_config(
        serde_json::to_string(&json!({
            "address" : {
            "id" : verify_user.email.clone(),
        }}))
        .expect("fail to make query"),
        base64::URL_SAFE,
    );

    let request_url = format!(
        "{address}/terra/wasm/v1beta1/contracts/{contract}/store?query_msg={query_msg}",
        address = lcd_address,
        contract = lot_contract,
        query_msg = encode_parsed
    );

    let response = match reqwest::blocking::get(&request_url) {
        Ok(response) => response,
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_CANT_FIND_EMAIL_FROM_CONTRACT),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    };

    let user_info = response
        .json::<ContractUser>()
        .expect("Fail to make user info by contract user info");

    if user_info.query_result.address != verify_user.wallet_address {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(
                    message_constants::MESSAGE_DIFFERENT_FROM_CONTRACT_WALLET_ADDRESS,
                ),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(message_constants::MESSAGE_OK),
            data: serde_json::to_value("").unwrap(),
        },
    }
}

pub fn sign_in_without_verify(conn: &Conn, verify_user: &VerifyUser) -> ResponseWithStatus {
    if auth_query::get_user_by_email(&conn, &verify_user.email).is_ok() {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_DUPLICATED_EMAIL),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let verify_email_hash =
        hash_generator::generate_expired_hash(&verify_user.email, Duration::hours(1));

    if auth_query::insert_user(&conn, {
        &user::UserAuthInfo {
            email: Some(verify_user.email.clone()),
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
                message: String::from(message_constants::MESSAGE_FAIL_INSERT_USER_AUTH_INFO),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let uuid = if let Ok(user) =
        auth_query::get_user_by_wallet_address(&conn, &verify_user.wallet_address)
    {
        user.seq.to_string()
    } else {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_INSERT_USER_AUTH_INFO),
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
    let user_auth_info =
        match auth_query::get_user_by_wallet_address(&conn, &insertable_user.wallet_address) {
            Ok(mut user_auth_info) => {
                user_auth_info.password = Some(hash_generator::generate_expired_hash(
                    &insertable_user.wallet_address,
                    Duration::days(1),
                ));
                user_auth_info.txHash = Some(insertable_user.txhash.clone());
                user_auth_info
            }
            Err(_) => {
                return ResponseWithStatus {
                    status_code: Status::BadRequest.code,
                    response: Response {
                        message: String::from(message_constants::MESSAGE_NOT_FOUND_USER_AUTH_INFO),
                        data: serde_json::to_value("").unwrap(),
                    },
                };
            }
        };

    if auth_query::update_user(&conn, &user_auth_info).is_err() {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_UPDATE_USER_AUTH_INFO),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    if user_query::insert_user(&conn, {
        &user::User {
            uuid: user_auth_info.seq,
            nickname: Some(insertable_user.nickname.clone()),
            profileImage: Some(insertable_user.profile_image.clone()),
            ..Default::default()
        }
    })
    .is_err()
    {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_INSERT_USER),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    if mail_system::send_mail(
        &user_auth_info.email.unwrap(),
        &MailSubjectType::UserPassword,
        &user_auth_info.password.unwrap(),
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
