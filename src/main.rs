#[macro_use]
extern crate rocket;
use diesel::pg::PgConnection;
use diesel::r2d2;
use once_cell::sync::OnceCell;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_api_rest::db;
use rocket_api_rest::db::person;
use rocket_api_rest::models::bad_request::BadRequest;
use rocket_api_rest::models::person::Person;
use rocket_api_rest::models::person::PostPerson;

static POOL: OnceCell<r2d2::Pool<r2d2::ConnectionManager<PgConnection>>> = OnceCell::new();

pub fn pool() -> &'static r2d2::Pool<r2d2::ConnectionManager<PgConnection>> {
    POOL.get_or_init(db::create_connection_pool)
}

//  curl -i -X POST -H 'Content-Type: application/json' -d '{"name": "Juanes", "age": 55}' http://127.0.0.1:8000/new_person
#[post("/new_person", format = "json", data = "<person_json>")]
fn post_person(person_json: Json<PostPerson>) -> Result<Json<Person>, Status> {
    let mut conn = pool().get().unwrap();

    match person::create(&mut conn, &person_json) {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status { code: 405 }),
    }
}

#[catch(405)]
fn resource_already_exists(status: Status, req: &Request) -> Json<BadRequest> {
    let bad_request = BadRequest {
        code: status.code,
        description: format!("Resource already exists : {}", req.uri()),
    };
    Json(bad_request)
}

// curl -i -X PUT 'http://127.0.0.1:8000/modify/1?name=Peponcio'
#[put("/modify/<id>?<name>&<age>")]
fn modify_person(id: i32, name: Option<String>, age: Option<i32>) -> Result<Json<Person>, Status> {
    let mut conn = pool().get().unwrap();

    match person::modify_person(&mut conn, id, age, name) {
        Ok(modified_person) => Ok(Json(modified_person)),
        Err(_) => Err(Status { code: 404 }),
    }
}

// curl -i -X DELETE 'http://127.0.0.1:8000/remove/1'
#[delete("/remove/<id>")]
fn remove_person(id: i32) -> Result<Json<Person>, Status> {
    let mut conn = pool().get().unwrap();

    match person::remove_person(&mut conn, id) {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status { code: 404 }),
    }
}

// curl http://127.0.0.1:8000/people/1
#[get("/people/<id>")]
fn people(id: i32) -> Result<Json<Person>, Status> {
    let mut conn = pool().get().unwrap();

    match person::get_person(&mut conn, id) {
        Ok(person) => Ok(Json(person)),
        Err(_) => Err(Status { code: 404 }),
    }
}

#[catch(404)]
fn resource_not_found(status: Status, req: &Request) -> Json<BadRequest> {
    let bad_request = BadRequest {
        code: status.code,
        description: format!("Resource not found : {}", req.uri()),
    };
    Json(bad_request)
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register(
            "/",
            catchers![
                can_not_parse_json,
                resource_not_found,
                resource_already_exists
            ],
        )
        .mount(
            "/",
            routes![people, post_person, modify_person, remove_person],
        )
}
