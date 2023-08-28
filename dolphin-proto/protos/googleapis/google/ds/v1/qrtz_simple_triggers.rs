//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "qrtz_simple_triggers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sched_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub trigger_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub trigger_group: String,
    pub repeat_count: i64,
    pub repeat_interval: i64,
    pub times_triggered: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
