use juniper::RootNode;

use crate::infrastructure::graphql::mutation_root::MutationRoot;
use crate::infrastructure::graphql::query_root::QueryRoot;

pub(crate) type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
