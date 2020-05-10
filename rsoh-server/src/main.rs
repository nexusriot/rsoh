#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/command")]
fn index() -> &'static str {
    // TODO: command input / DB
    // only for PoC
    "ls"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
