use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use uuid::Uuid;

use crate::{
    auth::auth::PlexoAuthToken,
    sdk::{
        member::{Member, MemberRole},
        utilities::DateTimeBridge,
    },
    system::core::Engine,
};

#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct Team {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub name: String,

    pub owner_id: Uuid,

    pub visibility: TeamVisibility,

    pub prefix: Option<String>,
}

#[ComplexObject]
impl Team {
    pub async fn owner(&self, ctx: &Context<'_>) -> Member {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();

        let member = sqlx::query!(
            r#"SELECT * FROM members WHERE id = $1"#,
            &self.owner_id
        )
        .fetch_one(&plexo_engine.pool)
        .await
        .unwrap();

        Member {
            id: member.id,
            created_at: DateTimeBridge::from_offset_date_time(member.created_at),
            updated_at: DateTimeBridge::from_offset_date_time(member.updated_at),
            name: member.name,
            email: member.email,
            github_id: member.github_id,
            google_id: member.google_id,
            photo_url: member.photo_url,
            role: MemberRole::from_optional_str(&member.role),
        }    
    }

    pub async fn members(&self, ctx: &Context<'_>) -> Vec<Member> {
        let auth_token = &ctx.data::<PlexoAuthToken>().unwrap().0;
        let plexo_engine = ctx.data::<Engine>().unwrap();
        let members = sqlx::query!(r#"SELECT 
        members.id,
        members.created_at,
        members.updated_at,
        members.name,
        members.email,
        members.github_id,
        members.google_id,
        members.photo_url,
        members.role 
        FROM members_by_teams JOIN members
         ON members_by_teams.member_id = members.id WHERE team_id = $1"#, &self.id).fetch_all(&plexo_engine.pool).await.unwrap();
        members
            .iter()
            .map(|r| Member {
                id: r.id,
                created_at: DateTimeBridge::from_offset_date_time(r.created_at),
                updated_at: DateTimeBridge::from_offset_date_time(r.updated_at),
                name: r.name.clone(),
                email: r.email.clone(),
                github_id: r.github_id.clone(),
                google_id: r.google_id.clone(),
                photo_url: r.photo_url.clone(),
                role: MemberRole::from_optional_str(&r.role),
            })
            .collect()
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TeamVisibility {
    None,
    Public,
    Private,
    Internal,
}

impl TeamVisibility {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Public" => Self::Public,
            "Private" => Self::Private,
            "Internal" => Self::Internal,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Public => "Public",
            Self::Private => "Private",
            Self::Internal => "Internal",
        }
    }
}
