//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_ds_task_group_queue")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub task_id: Option<i32>,
    pub task_name: Option<String>,
    pub group_id: Option<i32>,
    pub process_id: Option<i32>,
    pub priority: Option<i32>,
    pub status: Option<i32>,
    pub force_start: Option<i32>,
    pub in_queue: Option<i32>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}