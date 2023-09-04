use async_graphql::{Object, SimpleObject};
use poem::Result;

#[derive(Default)]
pub struct AuthMutation;

#[derive(SimpleObject)]
struct LoginResponse {
    token: String,
    member_id: String,
}

#[Object]
impl AuthMutation {
    async fn login(&self, email: String, password: String) -> Result<LoginResponse> {
        todo!()
    }

    async fn register(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> Result<LoginResponse> {
        todo!()
    }
}
