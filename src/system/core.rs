use async_graphql::Context;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{auth::engine::AuthEngine, sdk::member::Member};

use super::members::{MembersFilter, NewMemberPayload};

#[derive(Clone)]
pub struct Engine {
    pub pool: Pool<Postgres>,
    pub auth: AuthEngine,
}

impl Engine {
    pub fn new(pool: Pool<Postgres>, auth: AuthEngine) -> Self {
        Self { pool, auth }
    }

    pub fn me<'ctx>(&self, ctx: &'ctx Context) {
        let data = ctx.data::<String>().unwrap();

        println!("{}", data);
    }

    pub async fn get_member(&self, id: Uuid) -> Member {
        todo!()
    }

    pub async fn get_members(&self, filter: &MembersFilter) -> Vec<Member> {
        let m = sqlx::query!(
            r#"
            SELECT
                id,
                email,
                name,
                created_at,
                updated_at
            FROM members
            WHERE
                email = $1
            "#,
            filter.email
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        println!("{:#?}", m);

        vec![]
    }

    pub async fn create_member(&self, payload: &NewMemberPayload) -> Member {
        let m = sqlx::query!(
            r#"
            INSERT INTO members (email, name)
            VALUES ($1, $2)
            RETURNING
                id,
                email,
                name,
                created_at,
                updated_at
            "#,
            payload.email,
            payload.name
        );

        todo!()
    }

    pub async fn update_member(&self, member: Member) -> Member {
        todo!()
    }

    pub async fn delete_member(&self, member: Member) -> Member {
        todo!()
    }
}
