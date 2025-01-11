use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "teams")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub name: String,
    #[sea_orm(primary_key)]
    pub team: Team
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Hash, Copy, Serialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(1))")]
pub enum Team {
    #[sea_orm(string_value = "C")]
    Software,
    #[sea_orm(string_value = "M")]
    Mechanical,
    #[sea_orm(string_value = "E")]
    Electrical,
    #[sea_orm(string_value = "S")]
    Systems,
    #[sea_orm(string_value = "G")]
    Social,
    #[sea_orm(string_value = "A")]
    Admin,
}