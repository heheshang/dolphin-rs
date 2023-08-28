//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_process_task_relation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: Option<String>,
    pub project_code: Option<i64>,
    pub process_definition_code: Option<i64>,
    pub process_definition_version: Option<i32>,
    pub pre_task_code: Option<i64>,
    pub pre_task_version: Option<i32>,
    pub post_task_code: Option<i64>,
    pub post_task_version: Option<i32>,
    pub condition_type: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub condition_params: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
