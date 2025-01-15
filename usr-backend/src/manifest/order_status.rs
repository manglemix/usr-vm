use std::fmt::Display;

use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "order_status")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub instance_id: u32,
    pub order_id: u32,
    pub date: DateTime,
    pub status: Status
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

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}