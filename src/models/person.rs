use crate::schema::person;
use diesel::{Insertable, Queryable};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Queryable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = person)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable, PartialEq, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = person)]
pub struct PostPerson {
    pub name: String,
    pub age: i32,
}

impl From<Json<PostPerson>> for PostPerson {
    fn from(value: Json<PostPerson>) -> Self {
        value.into()
    }
}
