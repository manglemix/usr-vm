use sea_orm::entity::prelude::*;
use serde::Serialize;

use crate::scheduler;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub count: u32,
    pub unit_cost: Decimal,
    pub store_in: String,
    pub team: scheduler::Team,
    pub reason: String,
    pub vendor: String,
    pub link: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}