#[macro_use] extern crate rocket;

mod services;
mod fairings;
mod db;

use services::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(fairings::CORS)
        .mount("/", routes![create, get_problem, check, get_guesses])
}