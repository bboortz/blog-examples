use rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn test_not_found() {
	let client = Client::new(rocket()).unwrap();

    let res = client.get("/not_found")
	    .header(ContentType::JSON)
		.dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn test_alive() {
	let client = Client::new(rocket()).unwrap();

    let mut res = client.get("/")
	    .header(ContentType::JSON)
		.dispatch();
    let body = res.body_string().unwrap();
    assert_eq!(res.status(), Status::Ok);
    assert!(body.contains("OK"));
}

#[test]
fn test_post_hello() {
	let client = Client::new(rocket()).unwrap();

    let mut res = client.post("/hello")
        .header(ContentType::JSON)
        .body(r#"{ "name": "benni" }"#)
        .dispatch();

    assert_eq!(res.status(), Status::Ok);
	let body = res.body().unwrap().into_string().unwrap();
	assert!(body.contains("hello"));
}
