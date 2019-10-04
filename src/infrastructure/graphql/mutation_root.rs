use juniper::FieldResult;
use uuid::Uuid;

use crate::diesel::RunQueryDsl;
use crate::infrastructure::postgres::schema::users;
use crate::QueryContext;
use crate::infrastructure::graphql::user::NewUser;
use crate::domain::model::identity::user::User;

pub(crate) struct MutationRoot;

#[juniper::object(Context = QueryContext)]
impl MutationRoot {
    fn create_user(context: &QueryContext, new_user: NewUser) -> FieldResult<Option<User>> {
        let user = crate::infrastructure::postgres::user::NewUser::f
        let created_user = diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&context.database)
            .expect("Error saving new post");
        Ok(Some(created_user))
    }
}
