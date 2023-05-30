use sqlx::{Pool, Postgres};

use crate::{
    auth::engine::AuthEngine,
    llm::suggestions::AutoSuggestionsEngine,
    sdk::{
        member::{Member, MemberRole},
        utilities::DateTimeBridge,
    },
};

use super::subscriptions::SubscriptionManager;

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

    pub async fn get_member_by_github_id(&self, github_id: String) -> Option<Member> {
        sqlx::query!(
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
                github_id = $1
            ",
            github_id,
        )
        .fetch_one(&*self.pool)
        .await
        .ok()
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
            password_hash: None,
        })
    }

    pub async fn get_member_by_email(&self, email: String) -> Option<Member> {
        sqlx::query!(
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
                role,
                password_hash
            FROM members
            WHERE
                email = $1
            ",
            email,
        )
        .fetch_one(&*self.pool)
        .await
        .ok()
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
            password_hash: m.password_hash,
        })
    }

    pub async fn create_member_from_github(
        &self,
        email: String,
        name: String,
        github_id: String,
    ) -> Member {
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
            email,
            name,
            github_id,
        )
        .fetch_one(&*self.pool)
        .await
        .unwrap();

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
            password_hash: None,
        }
    }

    pub async fn create_member_from_email(
        &self,
        email: String,
        name: String,
        password_hash: String,
    ) -> Option<Member> {
        let m = sqlx::query!(
            "
            INSERT INTO members (email, name, password_hash)
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
            email,
            name,
            password_hash,
        )
        .fetch_one(&*self.pool)
        .await;

        match m {
            Ok(m) => Some(Member {
                id: m.id,
                email: m.email,
                name: m.name,
                created_at: DateTimeBridge::from_offset_date_time(m.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(m.updated_at),
                github_id: m.github_id,
                google_id: m.google_id,
                photo_url: m.photo_url,
                role: MemberRole::from_optional_str(&m.role),
                password_hash: None,
            }),
            Err(_) => None,
        }
    }

    pub async fn update_member(&self, _member: Member) -> Member {
        todo!()
    }

    pub async fn delete_member(&self, _member: Member) -> Member {
        todo!()
    }
}
