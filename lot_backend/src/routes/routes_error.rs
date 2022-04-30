use rocket::response::status;
use rocket::{http::Status, Request};
use rocket_contrib::json::Json;

use crate::model::response::Response;

#[catch(404)]
pub fn not_found(req: &Request) -> status::Custom<Json<Response>> {
    status::Custom(
        Status::NotFound,
        Json(Response {
            message: format!(
                "code is {}, {}",
                Status::NotFound.code,
                Status::NotFound.reason
            ),
            data: serde_json::to_value(format!("uri : {}", req.uri())).unwrap(),
        }),
    )
}

#[catch(500)]
pub fn internal_error() -> status::Custom<Json<Response>> {
    status::Custom(
        Status::InternalServerError,
        Json(Response {
            message: format!(
                "code is {}, {}",
                Status::InternalServerError.code,
                Status::InternalServerError.reason
            ),
            data: serde_json::to_value("").unwrap(),
        }),
    )
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> status::Custom<Json<Response>> {
    status::Custom(
        Status::UnprocessableEntity,
        Json(Response {
            message: format!(
                "code is {}, {}",
                Status::UnprocessableEntity.code,
                Status::UnprocessableEntity.reason
            ),
            data: serde_json::to_value(format!("uri : {}", req.uri())).unwrap(),
        }),
    )
}

// #[catch(default)]
// pub fn default(status: Status, req: &Request) -> String {
//     status::Custom(
//         Status::InternalServerError,
//         Json(
//             Response{
//                 message: String::from(message_constants::ERROR_DEFAULT),
//                 data: serde_json::to_value(format!("status : {}, uri : {}", status, req.uri())).unwrap(),
//             },
//         ),
//     )
// }
