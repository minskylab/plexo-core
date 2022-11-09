use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "Project")]
#[graphql(complex)]
#[graphql(name = "Project")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
    pub id: String,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub name: String,
    pub prefix: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
