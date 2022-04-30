#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

mod constants;
mod db;
mod model;
mod routes;
mod service;
mod util;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::connection::init_pool())
        .mount(
            "/",
            routes![
                routes::user_controller::index,
                routes::user_controller::get_user_by_wallet,
                routes::user_controller::verify_user_by_uuid_with_email_hash,
                routes::user_controller::sign_in_no_verify,
                routes::user_controller::sign_in_final,
            ],
        )
        .register(catchers![
            routes::routes_error::internal_error,
            routes::routes_error::not_found,
            routes::routes_error::unprocessable_entity,
            //routes::routes_error::default,
        ])
}
