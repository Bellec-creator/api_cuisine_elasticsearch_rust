use std::fmt::format;
use rocket::{Build, Rocket};
use rocket::tokio::time::{sleep, Duration};

#[macro_use] extern crate rocket;

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("waited for {} seconds", seconds)
}
#[get("/world")]
async fn world() -> &'static str {
    "Hello, world!"
}
#[get("/name")]
async fn name() -> &'static str {
    "Hello, name!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/hello", routes![world, name, delay])
}

#[cfg(test)]
mod test{
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut response = client.get("/hello/world").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
