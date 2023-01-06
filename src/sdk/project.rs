use sqlx::types::{time::PrimitiveDateTime, Uuid};

use crate::entities::task::Task;

use super::member::Member;

pub struct Project {
    id: Uuid,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,

    title: String,
    description: Option<String>,

    owner_id: Uuid,

    labels: Vec<String>,
}

impl Project {
    pub fn owner() -> Member {
        todo!()
    }

    pub fn members() -> Vec<Member> {
        todo!()
    }

    pub fn tasks() -> Vec<Task> {
        todo!()
    }
}
