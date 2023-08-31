use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::{GrpcRequest, GrpcResponse},
    core_status::app_status::AppStatus,
};
use entity::t_ds_user::{self};
use proto::ds_user::{
    ds_user_bean_service_server::DsUserBeanService,
    CreateDsUserBeanRequest,
    DeleteDsUserBeanRequest,
    DsUserBean,
    GetDsUserBeanRequest,
    ListDsUserBeansRequest,
    ListDsUserBeansResponse,
    UpdateDsUserBeanRequest,
};
use sea_orm::entity::prelude::*;

use super::service::DolphinRpcServer;

#[tonic::async_trait]
impl DsUserBeanService for DolphinRpcServer {
    async fn get_ds_user_bean(
        &self,
        req: GrpcRequest<GetDsUserBeanRequest>,
    ) -> GrpcResponse<DsUserBean> {
        let conn = &self.conn;
        let name = req.into_inner().name;
        let db_user: Option<t_ds_user::Model> = t_ds_user::Entity::find()
           // .column(t_ds_user::Column::UserName)
           .filter(t_ds_user::Column::UserName.eq(name))
           .into_model()
            .one(conn)
                   .await
                   .map_err(|_| tonic::Status::not_found("User not found"))?;
        match db_user {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                AppStatus::UserNotExist.into(),
            ))),
        }
    }

    async fn update_ds_user_bean(
        &self,
        req: GrpcRequest<UpdateDsUserBeanRequest>,
    ) -> GrpcResponse<DsUserBean> {
        let conn = &self.conn;
        if let Some(user) = req.into_inner().ds_user_bean {
            let db_user = t_ds_user::Entity::find_by_id(user.id)
                .one(conn)
                .await
                .map_err(|_| tonic::Status::not_found("User not found"))?;
            let mut db_user = db_user.unwrap();

            db_user.email = user.email;
            db_user.phone = user.phone;
            db_user.user_type = user.user_type;
            Ok(tonic::Response::new(db_user.into()))
        } else {
            Err(tonic::Status::not_found("User not found"))
            // Ok(tonic::Response::new(DsUserBean::default()))
        }
    }

    async fn list_ds_user_beans(
        &self,
        _req: GrpcRequest<ListDsUserBeansRequest>,
    ) -> GrpcResponse<ListDsUserBeansResponse> {
        todo!()
    }

    async fn create_ds_user_bean(
        &self,
        _request: GrpcRequest<CreateDsUserBeanRequest>,
    ) -> GrpcResponse<DsUserBean> {
        todo!()
    }

    async fn delete_ds_user_bean(
        &self,
        _request: GrpcRequest<DeleteDsUserBeanRequest>,
    ) -> GrpcResponse<()> {
        todo!()
    }
}
