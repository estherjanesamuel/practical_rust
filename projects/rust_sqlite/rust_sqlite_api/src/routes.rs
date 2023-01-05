use crate::models::{User, UserJson, UserNew};
use crate::Pool;

use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{web, Error, HttpResponse};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub async fn root() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::build(StatusCode::OK).body("Hello World, Rust!"))
}

pub async fn create_user(
    pool: Data,
    item: web::Json,
) -> Result<HttpResponse, Error> {
    Ok(web::block(move || new_user(pool, item))
        .await
        .map(|user| HttpResponse::Created().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn new_user(
    pool: Data,
    item: web::Json,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();

    match users
        .filter(name.eq(&item.name))
        .first::(&db_connection)
    {
        Ok(result) => Ok(result),
        Err(_) => {
            let new_user = UserNew {
                name: &item.name,
                address: &item.address,
                date_created: &format!("{}", chrono::Local::now().naive_local()),
            };

            insert_into(users)
                .values(&new_user)
                .execute(&db_connection)
                .expect("Error");

            let result = users.order(id.desc()).first(&db_connection).unwrap();
            Ok(result)
        }
    }
}

pub async fn get_users(pool: Data) -> Result<HttpResponse, Error> {
    Ok(list_users(pool)
        .await
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

async fn list_users(pool: Data) -> Result<Vec, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    let db_connection = pool.get().unwrap();
    let result = users.load(&db_connection)?;
    Ok(result)
}