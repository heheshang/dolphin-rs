//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "qrtz_blob_triggers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub sched_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub trigger_name: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub trigger_group: String,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))", nullable)]
    pub blob_data: Option<Vec<u8>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
