use async_graphql::Context;
use sqlx::{Pool, Postgres};

use crate::auth::engine::AuthEngine;

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
}
