#[cfg(test)]
use crate::rocket;
use rocket::local::blocking::Client;
use rocket::http::Status;


#[test]
fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let mut response = client.get("/hello/world").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
}
#[test]
fn delay(){
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let mut response = client.get("/hello/delay/2").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "waited for 2 seconds");
}
