use crate::schema::*;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub date_created: String,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub date_created: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub address: String,
}