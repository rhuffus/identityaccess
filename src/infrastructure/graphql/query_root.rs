use diesel::prelude::*;
use juniper::{FieldResult};

use crate::domain::model::identity::user::User;
use crate::infrastructure::postgres::schema::users::dsl::*;
use crate::QueryContext;
use uuid::Uuid;

pub struct QueryRoot;

#[juniper::object(Context = QueryContext)]
impl QueryRoot {

	fn users(context: &QueryContext) -> FieldResult<Vec<User>> {
		let result = users.load::<User>(&context.database);

		match result {
			Ok(result) => Ok(result),
			Err(err) => Err(err.into())
		}
	}

	fn user(context: &QueryContext, user_id: Uuid) -> FieldResult<Option<User>> {
		let user = users
			.filter(id.eq(&user_id))
			.limit(1)
			.get_result::<User>(&context.database);
		match user {
			Ok(user) => Ok(Some(user)),
			Err(err) => Err(err.into()),
		}
	}
}
