use uuid::Uuid;

pub struct User {
    id: i32,
    uuid: Uuid,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    email: String,
}

impl User {
    pub fn complete_name(&self) -> String {
        format!("{}! {}!", self.first_name, self.last_name)
    }

    pub fn change_first_name(&self, current_password: String, new_password: String) {

    }

    fn
}

pub struct UserCreatedEvent {
    first_name: String,
    last_name: String,
    username: String,
    password: String,
    email: String,
}
