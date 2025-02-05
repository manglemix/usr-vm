use sea_orm::entity::prelude::*;
use serde::Serialize;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "attendance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uid: u32,
    #[sea_orm(primary_key)]
    pub date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}