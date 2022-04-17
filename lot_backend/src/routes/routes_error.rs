
use rocket::{Request, http::Status};
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::{model::response::Response, constants::message_constants};

#[catch(404)]
pub fn not_found(req: &Request) -> status::Custom<Json<Response>> {
    status::Custom(
        Status::NotFound,
        Json(
            Response{ 
                message: String::from(message_constants::ERROR_INVALID_URI), 
                data: serde_json::to_value(format!("uri : {}", req.uri())).unwrap(),
            },
        ),
    )
}

#[catch(500)]
pub fn internal_error() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::InternalServerError,
        Json(
            Response{ 
                message: String::from(message_constants::ERROR_INTERNEAL_ERROR), 
                data: serde_json::to_value("").unwrap(),
            },
        ),
    )

}