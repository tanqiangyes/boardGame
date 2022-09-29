use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub uuid: Uuid,
}
