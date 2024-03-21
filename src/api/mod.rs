use rocket::{Build, Rocket};

mod v1;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    v1::mount(rocket)
}
