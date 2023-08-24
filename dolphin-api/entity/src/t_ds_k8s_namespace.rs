//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2


use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "t_ds_k8s_namespace")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub limits_memory: Option<i32>,
    pub namespace: Option<String>,
    pub online_job_num: Option<i32>,
    pub user_id: Option<i32>,
    pub pod_replicas: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((13, 4)))", nullable)]
    pub pod_request_cpu: Option<Decimal>,
    pub pod_request_memory: Option<i32>,
    #[sea_orm(column_type = "Decimal(Some((13, 4)))", nullable)]
    pub limits_cpu: Option<Decimal>,
    pub k8s: Option<String>,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}