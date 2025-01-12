use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::scheduler;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "orders")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub date: DateTime,
    pub status: Status,
    pub count: u32,
    pub unit_cost: Decimal,
    #[sea_orm(nullable)]
    pub store_in: Option<String>,
    pub team: scheduler::Team,
    pub reason: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Hash, Copy, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum Status {
    #[sea_orm(string_value = "N")]
    New,
    #[sea_orm(string_value = "S")]
    Submitted,
    #[sea_orm(string_value = "F")]
    Shipped,
    #[sea_orm(string_value = "D")]
    Delivered,
    #[sea_orm(string_value = "I")]
    InStorage,
}