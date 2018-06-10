#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket_contrib::{Json, Value};


#[derive(Serialize, Deserialize)]
struct HelloInput{
    name: String
}


#[get("/", format = "application/json")]
fn index() -> Json<Value> {
    Json(json!({
        "status": "OK",
    }))
}

#[post("/", format = "application/json", data = "<json_input>")]
fn hello(json_input: Json<HelloInput>) -> Json<Value> {
    Json(json!({
        "hello": json_input.name,
    }))
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
