use async_graphql::Context;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    auth::engine::AuthEngine,
    llm::suggestions::AutoSuggestionsEngine,
    sdk::{
        member::{Member, MemberRole},
        utilities::DateTimeBridge,
    },
};

use super::{
    members::{MembersFilter, NewMemberPayload},
    subscriptions::SubscriptionManager,
};

#[derive(Clone)]
pub struct Engine {
    pub pool: Box<Pool<Postgres>>,
    pub auth: AuthEngine,
    pub subscription_manager: SubscriptionManager,
    pub auto_suggestions_engine: AutoSuggestionsEngine,
}

impl Engine {
    pub fn new(pool: Pool<Postgres>, auth: AuthEngine) -> Self {
        let pool = Box::new(pool);
        let subscription_manager = SubscriptionManager::new();
        let auto_suggestions_engine = AutoSuggestionsEngine::new(pool.clone());

        Self {
            pool,
            auth,
            subscription_manager,
            auto_suggestions_engine,
        }
    }

    pub fn me(&self, ctx: &Context) {
        let data = ctx.data::<String>().unwrap();

        println!("{}", data);
    }

    pub async fn get_member(&self, _id: Uuid) -> Member {
        todo!()
    }

    pub async fn get_members(&self, filter: &MembersFilter) -> Vec<Member> {
        let m = sqlx::query!(
            "
            SELECT
                id,
                email,
                name,
                created_at,
                updated_at,
                github_id,
                google_id,
                photo_url,
                role
            FROM members
            WHERE
                name = COALESCE($1, name) OR
                email = COALESCE($2, email) OR
                role = COALESCE($3, role) OR
                github_id = COALESCE($4, github_id) OR
                google_id = COALESCE($5, google_id)
            ",
            filter.name,
            filter.email,
            filter.role.map(|r| r.to_str()),
            filter.github_id,
            filter.google_id,
        )
        .fetch_all(&*self.pool)
        .await
        .unwrap();

        m.iter()
            .map(|m| Member {
                id: m.id,
                email: m.email.clone(),
                name: m.name.clone(),
                created_at: DateTimeBridge::from_offset_date_time(m.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(m.updated_at),
                github_id: m.github_id.as_ref().map(|id| id.to_string()),
                google_id: m.google_id.as_ref().map(|id| id.to_string()),
                photo_url: m.photo_url.clone(),
                role: MemberRole::from_optional_str(&m.role),
            })
            .collect::<Vec<Member>>()
    }

    pub async fn create_member(&self, payload: &NewMemberPayload) -> Member {
        let m = sqlx::query!(
            "
            INSERT INTO members (email, name, github_id)
            VALUES ($1, $2, $3)
            RETURNING
                id,
                email,
                name,
                created_at,
                updated_at,
                github_id,
                google_id,
                photo_url,
                role
            ",
            payload.email,
            payload.name,
            payload.auth_id,
        )
        .fetch_one(&*self.pool)
        .await
        .unwrap();

        // println!("{:?}", m);

        Member {
            id: m.id,
            email: m.email,
            name: m.name,
            created_at: DateTimeBridge::from_offset_date_time(m.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(m.updated_at),
            github_id: m.github_id,
            google_id: m.google_id,
            photo_url: m.photo_url,
            role: MemberRole::from_optional_str(&m.role),
        }
    }

    pub async fn update_member(&self, _member: Member) -> Member {
        todo!()
    }

    pub async fn delete_member(&self, _member: Member) -> Member {
        todo!()
    }
}
