use crate::graphql::resources::member::Member;

use super::accessor::{Accessor, ByPK, Where};

struct MemberAccessor {
    member: &'static Member,
}

struct MemberWhere {
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
    github_id: Option<String>,
    google_id: Option<String>,
}

impl ByPK<MemberAccessor> for MemberAccessor {
    fn get_by_pk(&self) -> &str {
        todo!()
    }
}

impl Where<MemberAccessor> for MemberWhere {
    fn filter(&self, filter_name: &str, filter_value: &str) -> MemberAccessor {
        todo!()
    }
}

impl Accessor<MemberAccessor, MemberAccessor, MemberWhere> for MemberAccessor {
    fn select(where_conditions: MemberWhere) -> Vec<MemberAccessor> {
        // where_conditions.
        // where_conditions.
        todo!()
    }

    fn aggregate() -> Vec<MemberAccessor> {
        todo!()
    }

    fn select_by_pk(pk: Self) -> MemberAccessor {
        // pk.get_by_pk(pk);
        todo!()
    }

    fn insert(data: Vec<MemberAccessor>) -> Vec<MemberAccessor> {
        todo!()
    }

    fn insert_one(data: MemberAccessor) -> MemberAccessor {
        todo!()
    }

    fn update(data: Vec<MemberAccessor>) -> Vec<MemberAccessor> {
        todo!()
    }

    fn update_one(data: MemberAccessor) -> MemberAccessor {
        todo!()
    }

    fn update_by_pk(pk: Self) -> MemberAccessor {
        todo!()
    }

    fn delete(data: Vec<MemberAccessor>) -> Vec<MemberAccessor> {
        todo!()
    }

    fn delete_by_pk(pk: Self) -> MemberAccessor {
        todo!()
    }
}
