#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;

#[macro_use]
extern crate rocket;
#[macro_use]
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

pub fn test_generate_hash() {
    util::hash_generator::generate_hash("test_mail".to_string());
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
    .manage(db::connection::init_pool())
    .mount("/",
        routes![
            routes::user_manage::hello,
            routes::user_manage::db,
            routes::user_manage::get_user_by_wallet,
            routes::user_manage::verify_user_by_uuid_with_eamil_hash,
            ],
    )
}