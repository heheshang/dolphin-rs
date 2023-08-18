//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_dq_task_statistics_value")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub process_definition_id: i32,
    pub task_instance_id: Option<i32>,
    pub rule_id: i32,
    pub unique_code: String,
    pub statistics_name: Option<String>,
    #[sea_orm(column_type = "Double", nullable)]
    pub statistics_value: Option<f64>,
    pub data_time: Option<DateTime>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
