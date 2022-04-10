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

mod util;
mod db;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .manage(db::connection::init_pool())
    .mount("/",
        routes![
            routes::user_manage::index,
            routes::user_manage::hello,
            routes::user_manage::db,
            routes::user_manage::get_user_by_wallet,
            routes::user_manage::verify_user_by_uuid_with_eamil_hash,
            routes::user_manage::sign_in_no_verify,
            routes::user_manage::sign_in_final,
            ],
    )
}