use crate::constants::{message_constants, url_constants};
use crate::db::connection::Conn;
use crate::db::model::user;
use crate::db::query::*;
use crate::model::contract_dto::ContractUser;
use crate::model::response::{self, Response, ResponseWithStatus};
use crate::model::user_dto::{InsertableUser, VerifyUser};
use crate::util::mail_system::MailSubjectType;
use crate::util::{hash_generator, mail_system};

use dotenv::dotenv;
use rocket::http::Status;
use serde_json::json;
use std::env;
use time::Duration;

const COUNT_INIT: &str = "0";

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
                data: serde_json::to_value(response::ResponseUser::get_response_user_from_userdb(
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
    let contract_info = match get_contract_info(base64::encode_config(
        serde_json::to_string(&json!({
            "address" : {
                "id" : verify_user.email.clone(),
        }}))
        .expect("fail to make query"),
        base64::URL_SAFE,
    )) {
        Ok(contract_info) => contract_info,
        Err(response) => return response,
    };
    if contract_info.query_result.address != verify_user.wallet_address {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(
                    message_constants::MESSAGE_CANT_FIND_MATCHED_INFO_FROM_CONTRACT,
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
        hash_generator::generate_expired_hash(&COUNT_INIT.to_string(), Duration::hours(1));

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
                    &COUNT_INIT.to_string(),
                    Duration::hours(1),
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

fn get_contract_info(request_query: String) -> Result<ContractUser, ResponseWithStatus> {
    dotenv().ok();
    let lcd_address = env::var("LCD_ADDRESS").expect("LCD_ADDRESS must be set");
    let lot_contract = env::var("LOT_CONTRACT").expect("LOT_CONTRACT must be set");

    let request_url = format!(
        "{address}/terra/wasm/v1beta1/contracts/{contract}/store?query_msg={query_msg}",
        address = lcd_address,
        contract = lot_contract,
        query_msg = request_query
    );

    match reqwest::blocking::get(&request_url) {
        Ok(response) => Ok(response
            .json::<ContractUser>()
            .expect("Fail to make user info by contract user info")),
        Err(_) => {
            Err(ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(
                        message_constants::MESSAGE_CANT_FIND_MATCHED_INFO_FROM_CONTRACT,
                    ),
                    data: serde_json::to_value("").unwrap(),
                },
            })
        }
    }
}

pub fn token_reissuance(conn: &Conn, wallet_address: String) -> ResponseWithStatus {
    let contract_info = match get_contract_info(base64::encode_config(
        serde_json::to_string(&json!({
            "id" : {
            "address" : wallet_address,
        }}))
        .expect("fail to make query"),
        base64::URL_SAFE,
    )) {
        Ok(contract_info) => contract_info,
        Err(response) => return response,
    };

    let user_auth_info = match auth_query::get_user_by_email(&conn, &contract_info.query_result.id)
    {
        Ok(user_auth_info) => user_auth_info,
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::NotFound.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND_USER_AUTH_INFO),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    };

    let password_token = match &user_auth_info.password {
        Some(password_token) => password_token,
        None => {
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_ISSUED_PASSWORD),
                    data: serde_json::to_value("").unwrap(),
                },
            };
        }
    };

    let auth_count = match hash_generator::decode_token(&password_token) {
        Ok(count) => count.parse::<i64>().unwrap(),
        Err(err_msg) => {
            println!("fail to decode password : {}", err_msg);
            return ResponseWithStatus {
                status_code: Status::BadRequest.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND_USER_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                },
            };
        }
    };

    if contract_info.query_result.count <= auth_count {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_CAN_NOT_RESET_PASSWORD),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    let mut user_auth_info = user_auth_info;
    user_auth_info.password = Some(hash_generator::generate_expired_hash(
        &contract_info.query_result.count.to_string(),
        Duration::hours(1),
    ));

    if auth_query::update_user(&conn, &user_auth_info).is_err() {
        return ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_UPDATE_USER_AUTH_INFO),
                data: serde_json::to_value("").unwrap(),
            },
        };
    }

    match token_query::get_user_by_uuid(&conn, &user_auth_info.seq)
    {
        Ok(mut user_token) => {
            user_token.token=Some("".to_string());
            if token_query::update_user_token(&conn, &user_token).is_err() {
                return ResponseWithStatus {
                    status_code: Status::BadRequest.code,
                    response: Response {
                        message: String::from(message_constants::MESSAGE_FAIL_UPDATE_USER_TOKEN),
                        data: serde_json::to_value("").unwrap(),
                    },
                }
            }
        },
        Err(_) => {
            return ResponseWithStatus {
                status_code: Status::NotFound.code,
                response: Response {
                    message: String::from(message_constants::MESSAGE_NOT_FOUND_USER_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                },
            }
        }
    };


    //다시 메일 보내주기
    match mail_system::send_mail(
        &user_auth_info.email.unwrap(),
        &MailSubjectType::UserPassword,
        &user_auth_info.password.unwrap(),
    ) {
        Ok(_) => ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_OK),
                data: serde_json::to_value("").unwrap(),
            },
        },
        Err(_) => ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_FAIL_SEND_MAIL),
                data: serde_json::to_value("").unwrap(),
            },
        },
    }
}
