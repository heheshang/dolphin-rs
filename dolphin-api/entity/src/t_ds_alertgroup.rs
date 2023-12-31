//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_ds_alertgroup")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub alert_instance_ids: Option<String>,
    pub create_user_id: Option<i32>,
    #[sea_orm(unique)]
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
