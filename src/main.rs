#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::request::{FromRequest, Request, Outcome};
use rocket::serde::json::{Value, json};
use rocket::response::status;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;




pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header:  &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2  {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }
        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth>{
        let decoded = STANDARD::decode(base64_string).ok()?;
        let  decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();  // split username:password

        // If exactly username and password pair is present
        if split.len() != 2{
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth { username, password })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error  = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self,Self::Error>{
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header)  = auth_header  {
            if let Some(auth)  = Self::from_authorization_header(auth_header){
                return  Outcome::Success(auth)
            }
        }
        Outcome::Failure((Status::Unauthorized,()))
    }
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

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
#[post("/rustaceans", format = "json")]  // curl 127.0.0.1:8000/rustaceans -X POST -H "Content-Type: application/json"
fn create_rustacean() -> Value {
    json!([{"id":3, "name":"Marcos"}])
}  
#[put("/rustaceans/<id>", format = "json")]   // curl 127.0.0.1:8000/rustaceans -X PUT -H "Content-Type: application/json"
fn update_rustaceans(id: i32) -> Value {
    json!([{"id":id, "name":"Saksham Tewari"}])
}
#[delete("/rustaceans/<_id>")]   // curl 127.0.0.1:8000/rustaceans -X DELETE -I
fn delete_rustaceans(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main(){
    let _ = rocket::build().mount("/", routes![hello_rustaceans,get_rustaceans, view_rustacean, create_rustacean,update_rustaceans,delete_rustaceans]).register("/", catchers![not_found]).launch().await;
}