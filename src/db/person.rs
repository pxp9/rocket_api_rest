use crate::models::person::{Person, PostPerson};
use crate::schema::person;
use diesel::result::Error as DieselError;
use diesel::{self, QueryDsl};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    DieselError(#[from] DieselError),
    PersonNotFoundError,
    ModifyError,
    PersonNotDeletedError,
}

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::DieselError(err) => write!(f, "{}", err),
            DbError::PersonNotFoundError => write!(f, "Person not found!"),
            DbError::ModifyError => write!(f, "Modify error!"),
            DbError::PersonNotDeletedError => write!(f, "Can not delete person!"),
        }
    }
}

pub fn create(conn: &mut PgConnection, person: &PostPerson) -> Result<Person, DbError> {
    let vec: Vec<Person> = diesel::insert_into(person::table)
        .values(person)
        .get_results(conn)?;
    if vec.len() == 1 {
        Ok(vec[0].clone())
    } else {
        Err(DbError::PersonNotFoundError)
    }
}

pub fn get_person(conn: &mut PgConnection, id: i32) -> Result<Person, DbError> {
    let vec: Vec<Person> = person::table.filter(person::id.eq(id)).load(conn)?;

    if vec.len() == 1 {
        Ok(vec[0].clone())
    } else {
        Err(DbError::PersonNotFoundError)
    }
}

pub fn remove_person(conn: &mut PgConnection, id: i32) -> Result<Person, DbError> {
    let vec = diesel::delete(person::table)
        .filter(person::id.eq(id))
        .load::<Person>(conn)?;

    if vec.len() == 1 {
        Ok(vec[0].clone())
    } else {
        Err(DbError::PersonNotDeletedError)
    }
}

pub fn modify_person(
    conn: &mut PgConnection,
    id: i32,
    age_opt: Option<i32>,
    name_opt: Option<String>,
) -> Result<Person, DbError> {
    let query = diesel::update(person::table).filter(person::id.eq(id));

    let vec = match age_opt {
        Some(age) => match name_opt {
            Some(name) => query
                .set((person::age.eq(age), person::name.eq(name)))
                .load::<Person>(conn)?,
            None => query.set(person::age.eq(age)).load(conn)?,
        },
        None => match name_opt {
            Some(name) => query.set(person::name.eq(name)).load(conn)?,
            None => {
                return Err(DbError::ModifyError);
            }
        },
    };

    if vec.len() == 1 {
        Ok(vec[0].clone())
    } else {
        Err(DbError::PersonNotFoundError)
    }
}
