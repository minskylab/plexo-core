// use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, async_graphql::SimpleObject)]
// #[sea_orm(table_name = "Member")]
// // #[graphql(complex)]
// #[graphql(name = "Member")]
// pub struct Model {
//     #[sea_orm(primary_key, auto_increment = false, unique)]
//     pub id: String,
//     pub created_at: Option<DateTime>,
//     pub updated_at: Option<DateTime>,
//     pub name: String,
//     #[sea_orm(unique)]
//     pub email: String,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {
//     #[sea_orm(has_many = "super::task::Entity")]
//     Tasks,
// }

// impl Related<super::task::Entity> for Entity {
//     fn to() -> RelationDef {
//         Relation::Tasks.def()
//     }
// }

// impl ActiveModelBehavior for ActiveModel {}
