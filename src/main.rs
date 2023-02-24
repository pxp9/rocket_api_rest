#[macro_use]
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::r2d2;
use once_cell::sync::OnceCell;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_example::db;
use rocket_example::db::person;
use rocket_example::models::bad_request::BadRequest;
use rocket_example::models::person::Person;
use rocket_example::models::person::PostPerson;
use serde::{Deserialize, Serialize};

static POOL: OnceCell<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>> = OnceCell::new();

pub fn pool() -> &'static r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    POOL.get_or_init(db::create_connection_pool)
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    #[serde(rename(serialize = "person", deserialize = "person"))]
    Person(Person),
    #[serde(rename(serialize = "bad_request", deserialize = "bad_request"))]
    BadRequest(BadRequest),
}

//  curl -i -X POST -H 'Content-Type: application/json' -d '{"name": "Juanes", "age": 55}' http://127.0.0.1:8000/new_person
#[post("/new_person", format = "json", data = "<person_json>")]
fn post_person(person_json: Json<PostPerson>) -> Json<Response> {
    let mut conn = pool().get().unwrap();

    match person::create(&mut conn, &person_json) {
        Ok(person) => Json(Response::Person(person)),
        Err(e) => {
            let bad_request = BadRequest {
                code: 400,
                description: format!("Can not insert user : {:?}", e),
            };
            Json(Response::BadRequest(bad_request))
        }
    }
}

// curl -i -X PUT http://127.0.0.1:8000/modify/1\?name\=Peponcio\&age\=22
#[put("/modify/<id>?<name>&<age>")]
fn modify_person(id: i32, name: Option<String>, age: Option<i32>) -> Json<Response> {
    let mut conn = pool().get().unwrap();

    match person::modify_person(&mut conn, id, age, name) {
        Ok(modified_person) => Json(Response::Person(modified_person)),
        Err(err) => {
            let bad_request = BadRequest {
                code: 400,
                description: format!("Can not update person : {:?}", err),
            };
            Json(Response::BadRequest(bad_request))
        }
    }
}

#[catch(422)]
fn can_not_parse_json(status: Status, req: &Request) -> Json<BadRequest> {
    let bad_request = BadRequest {
        code: status.code,
        description: format!(
            "HTTP status {}\n\nCan not parse json from {} {} {}",
            status,
            req.method(),
            req.uri(),
            req.content_type().unwrap().media_type()
        ),
    };

    Json(bad_request)
}

// curl http://127.0.0.1:8000/people/1
#[get("/people/<id>")]
fn people(id: i32) -> Json<Response> {
    let mut conn = pool().get().unwrap();

    match person::get_person(&mut conn, id) {
        Ok(person) => Json(Response::Person(person)),
        Err(_) => {
            let bad_request = BadRequest {
                code: 400,
                description: String::from("User not found"),
            };
            Json(Response::BadRequest(bad_request))
        }
    }
}

#[launch]
fn rocket() -> _ {
    let p1 = PostPerson {
        name: String::from("Pepe"),
        age: 21,
    };
    let p2 = PostPerson {
        name: String::from("Momo"),
        age: 54,
    };

    let mut conn = pool().get().unwrap();

    if let Err(_) = person::get_person(&mut conn, 1) {
        person::create(&mut conn, &p1).unwrap();
        person::create(&mut conn, &p2).unwrap();
    }

    rocket::build()
        .register("/", catchers![can_not_parse_json])
        .mount("/", routes![people, post_person, modify_person])
}
