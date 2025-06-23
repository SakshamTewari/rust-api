#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};
use rocket::response::status;

#[get("/")]
fn hello_rustaceans() -> Value {
    json!("Hello, Rustaceans!")
}

#[get("/rustaceans")]
fn get_rustaceans() -> Value {
    json!([{"id":1, "name":"Saksham"}, {"id":2, "name":"Tewari"}])
}
#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Value {
    json!([{"id": id, "name":"Saksham Tewari"}])
}
#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Value {
    json!([{"id":3, "name":"Marcos"}])
}
#[put("/rustaceans/<id>", format = "json")]
fn update_rustaceans(id: i32) -> Value {
    json!([{"id":id, "name":"Saksham Tewari"}])
}
#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main(){
    let _ = rocket::build().mount("/", routes![hello_rustaceans,get_rustaceans, view_rustacean, create_rustacean,update_rustaceans,delete_rustaceans]).launch().await;
}