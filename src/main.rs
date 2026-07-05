///Inner dev loop
/// * Make changes
/// * Compile the application
/// * Run tests
/// * Run the application
// okay.. there are two parts - i.e two user stories
// 1. for the user to be able to subscribe to the service with their mail id
// 2. for the author to be able to send mails to the users
// both of these are meant to announce/to be aware when new content is published

use rocket::{catch, catchers, get, launch, post, routes, Request};
use rocket_dyn_templates::{context, Template};
use rusqlite::Connection;
#[get("/")]
fn index() -> Template {
    Template::render("index", context!{})
}

#[get("/alternate")]
fn alternate() -> &'static str {
    "alternate world!"
}

#[get("/<name>")]
fn spit_name(name: &str) -> String {
    format!("The name is {}", name)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
   return format!("{} is not a valid path.", req.uri())
}


#[launch]
fn rocket() -> _ {
    let path = "records";
    let connection = Connection::open(path).unwrap();
    connection.prepare("select * from subscriptions").unwrap();
     rocket::build().mount("/", routes![index, alternate, spit_name])
        .register("/",catchers![not_found])
        .attach(Template::fairing())
}
