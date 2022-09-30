use anthow::Result;
use uuid::Uuid;

pub struct User {
    pub uuid: Uuid,
}

pub fn login(user: &User) -> Result<bool> {
    Ok(true)
}
