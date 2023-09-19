use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{
    auth::engine::AuthEngine,
    llm::suggestions::AutoSuggestionsEngine,
    sdk::{
        activity::{Activity, ActivityOperationType, ActivityResourceType},
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
        .ok()
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
        .ok()
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

        m.map(|m| Member {
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
        })
        .ok()
    }

    pub async fn set_organization_name(&self, name: String) {
        let current_organization = sqlx::query!(
            r#"
            SELECT id, name
            FROM self
            "#
        )
        .fetch_one(&*self.pool)
        .await;

        match current_organization {
            Ok(_) => {
                // sqlx::query!(
                //     r#"
                //     UPDATE self
                //     SET name = $1
                //     WHERE id = $2
                //     "#,
                //     name,
                //     org.id,
                // )
                // .execute(&*self.pool)
                // .await
                // .unwrap();
            }
            Err(_) => {
                sqlx::query!(
                    r#"
                    INSERT INTO self (name)
                    VALUES ($1)
                    "#,
                    name,
                )
                .execute(&*self.pool)
                .await
                .unwrap();
            }
        }
    }

    pub async fn record_activity(
        &self,
        operation: ActivityOperationType,
        resource_type: ActivityResourceType,
        resource_id: Uuid,
        member_id: Uuid,
    ) -> Option<Activity> {
        sqlx::query!(
            r#"
            INSERT INTO activity (operation, resource_type, resource_id, member_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            operation.to_string(),
            resource_type.to_string(),
            resource_id,
            member_id,
        )
        .fetch_one(&*self.pool)
        .await
        .ok()
        .map(|res| Activity {
            id: res.id,
            created_at: DateTimeBridge::from_offset_date_time(res.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(res.updated_at),
            operation,
            resource_type,
            resource_id,
            member_id,
        })
    }
}
