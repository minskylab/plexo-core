use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    // seaography::macros::Filter,
)]
#[sea_orm(table_name = "Task")]
// #[graphql(complex)]
#[graphql(name = "Task")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        unique,
        default_value = "uuid_generate_v4()"
    )]
    pub id: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::member::Entity")]
    Members,
}

impl Related<super::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Members.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
