use uuid::Uuid;

#[derive(diesel::Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(diesel::Insertable)]
pub struct NewUser {
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl From<crate::domain::model::identity::user::User> for NewUser{
    fn from(_: User) -> Self {
        unimplemented!()
    }
}
