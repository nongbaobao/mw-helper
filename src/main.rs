#[path ="models.rs"]
mod models;

#[path ="./database/app.rs"]
mod app;


#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

use crate::app::connect;

#[launch]
async fn rocket() -> _ {
    connect().await.unwrap();
    rocket::build().mount("/", routes![index])

}