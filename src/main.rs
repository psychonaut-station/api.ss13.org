use rocket::{get, routes};

mod api;
mod byond;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build().mount("/", routes![index]);
    let rocket = api::mount(rocket);

    rocket.launch().await?;

    Ok(())
}
