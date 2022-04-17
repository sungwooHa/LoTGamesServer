use crate::constants::url_constants;
use crate::db::connection::Conn;
use crate::model::response::Response;
use crate::model::user_dto::{InsertableUser, VerifyUser};
use crate::service::user_service;

use rocket::http::Status;
use rocket::response::{status, Redirect};
use rocket_contrib::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Application successfully started!"
}

#[allow(non_snake_case)]
#[get("/users/address/<walletAddress>")]
pub fn get_user_by_wallet(conn: Conn, walletAddress: String) -> status::Custom<Json<Response>> {
    let response = user_service::get_user_by_wallet(&conn, &walletAddress);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[allow(non_snake_case)]
#[get("/users/verify/<uuid>/<verifyEmailHash>")]
pub fn verify_user_by_uuid_with_email_hash(
    conn: Conn,
    uuid: i64,
    verifyEmailHash: String,
) -> status::Custom<Json<Response>> {
    let response =
        user_service::verify_user_by_uuid_with_email_hash(&conn, &uuid, &verifyEmailHash);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[allow(non_snake_case)]
#[post("/users", format = "application/json", data = "<verifyUser>")]
pub fn sign_in_no_verify(
    conn: Conn,
    verifyUser: Json<VerifyUser>,
) -> status::Custom<Json<Response>> {
    let response = user_service::sign_in_without_verify(&conn, &verifyUser);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[allow(non_snake_case)]
#[put("/users", format = "application/json", data = "<insertableUser>")]
pub fn sign_in_final(
    conn: Conn,
    insertableUser: Json<InsertableUser>,
) -> Result<Redirect, status::Custom<Json<Response>>> {
    let response = user_service::sign_in_final(&conn, &insertableUser);

    match Status::from_code(response.status_code).unwrap() {
        Status::MovedPermanently => Ok(Redirect::moved(url_constants::LOT_URL)),
        _ => Err(status::Custom(
            Status::from_code(response.status_code).unwrap(),
            Json(response.response),
        )),
    }
}
