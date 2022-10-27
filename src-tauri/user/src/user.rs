use anthow::Result;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,            // a uuid for this user, unique, for login, auto-generated
    pub name: String,          // name, for login, unique
    pub age: Option<u64>,      // age
    password: String,          // password, just used in login
    pub phone: Option<String>, // phone, for login and other things , unique
    pub created_at: u64,       // created_at
    pub updated_at: u64,       // updated_at
    pub is_abandon: bool,      //the user abandon or not
}

pub fn login_by_name(name: String) -> Result {
    Ok(true)
}
