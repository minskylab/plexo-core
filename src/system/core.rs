use async_graphql::Context;
use sqlx::{Pool, Postgres};

pub struct Engine {
    pool: Pool<Postgres>,
}

impl Engine {
    pub fn new_with_pool(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub fn me<'ctx>(&self, ctx: &'ctx Context) {
        let data = ctx.data::<String>().unwrap();

        println!("{}", data);
    }
}
