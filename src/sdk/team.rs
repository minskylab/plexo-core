// use async_graphql::{ComplexObject, Context, Enum, SimpleObject};
// use chrono::{DateTime, Utc};

// use uuid::Uuid;

// use super::member::Member;

// #[derive(SimpleObject, Clone)]
// #[graphql(complex)]
// pub struct Team {
//     pub id: Uuid,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,

//     pub name: String,

//     pub owner_id: Uuid,

//     pub visibility: TeamVisibility,
// }

// #[ComplexObject]
// impl Team {
//     pub async fn owner(&self, ctx: &Context<'_>) -> Member {
//         todo!()
//     }

//     pub async fn members(&self, ctx: &Context<'_>) -> Vec<Member> {
//         todo!()
//     }
// }

// #[derive(Enum, Copy, Clone, Eq, PartialEq)]
// pub enum TeamVisibility {
//     None,
//     Public,
//     Private,
//     Internal,
// }

// impl TeamVisibility {
//     pub fn from_optional_str(s: &Option<String>) -> Self {
//         match s {
//             Some(s) => Self::from_str(s.as_str()),
//             None => Self::None,
//         }
//     }

//     pub fn from_str(s: &str) -> Self {
//         match s {
//             "None" => Self::None,
//             "Public" => Self::Public,
//             "Private" => Self::Private,
//             "Internal" => Self::Internal,
//             _ => Self::None,
//         }
//     }

//     pub fn to_str(&self) -> &'static str {
//         match self {
//             Self::None => "None",
//             Self::Public => "Public",
//             Self::Private => "Private",
//             Self::Internal => "Internal",
//         }
//     }
// }
