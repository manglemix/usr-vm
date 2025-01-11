use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "availabilities")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub name: String,
    /// duration from 12 AM Monday, in 15 minute units
    /// eg. 1 = 12:15 AM, 2 = 12:30 AM, 3 = 12:45 AM, 4 = 1:00 AM
    #[sea_orm(primary_key)]
    pub time: u16
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}