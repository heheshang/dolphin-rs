use super::service::DolphinRpcServer;
use dolphin_common::{
    core_error::error::DolphinErrorInfo,
    core_results::results::{GrpcRequest, GrpcResponse},
    core_status::app_status::AppStatus,
};
use entity::t_ds_session::{self};
use proto::ds_session::{
    ds_session_bean_service_server::DsSessionBeanService,
    DsSessionBean,
    GetDsSessionBeanByIdRequest,
    GetDsSessionBeanUserIdRequest,
    GetDsSessionBeanUserIdResponse,
};
use sea_orm::entity::prelude::*;

#[tonic::async_trait]
impl DsSessionBeanService for DolphinRpcServer {
    async fn list_ds_session_beans(
        &self,
        _req: tonic::Request<proto::ds_session::ListDsSessionBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_session::ListDsSessionBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::GetDsSessionByIRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::CreateDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_session_bean(
        &self,
        req: tonic::Request<proto::ds_session::UpdateDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        let ds_session = req.get_ref().ds_session_bean.clone().unwrap();
        let id = ds_session.id.clone();
        let parse_from_str = DateTime::parse_from_str;
        let current_time = chrono::prelude::Local::now().naive_local();
        let model = t_ds_session::ActiveModel {
            id: sea_orm::ActiveValue::Set(ds_session.id),
            user_id: sea_orm::ActiveValue::Set(Some(ds_session.user_id)),
            ip: sea_orm::ActiveValue::Set(ds_session.ip),
            last_login_time: sea_orm::ActiveValue::Set(Some(
                parse_from_str(&ds_session.last_login_time.unwrap(), "%Y-%m-%d %H:%M:%S")
                    .unwrap_or(current_time),
            )),
        };

        let conn = &self.conn;
        let res = t_ds_session::Entity::update(model.clone())
            .filter(t_ds_session::Column::Id.eq(id))
            .exec(conn)
            .await
            .map_err(|_| tonic::Status::internal("session  update failed"))?;
        Ok(tonic::Response::new(res.into()))
    }

    async fn delete_ds_session_bean(
        &self,
        request: tonic::Request<proto::ds_session::DeleteDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        let conn = &self.conn;
        let id = request.into_inner().id;
        let del_res = t_ds_session::Entity::delete_by_id(id)
            .exec(conn)
            .await
            .map_err(|_| tonic::Status::not_found("sessionId  not found"))?;
        if del_res.rows_affected > 0 {
            Ok(tonic::Response::new(()))
        } else {
            Err(tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                AppStatus::LoginSessionFailed.into(),
            )))
        }
    }

    async fn get_ds_session_by_id(
        &self,
        request: GrpcRequest<GetDsSessionBeanByIdRequest>,
    ) -> GrpcResponse<DsSessionBean> {
        let conn = &self.conn;
        let id = request.into_inner().id;
        let ds_session: Option<t_ds_session::Model> = t_ds_session::Entity::find()
            .filter(t_ds_session::Column::Id.eq(id))
            .into_model()
            .one(conn)
            .await
            .map_err(|_| tonic::Status::not_found("sessionId  not found"))?;
        match ds_session {
            Some(v) => Ok(tonic::Response::new(v.into())),
            None => Err(tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                AppStatus::LoginSessionFailed.into(),
            ))),
        }
    }

    async fn get_ds_session_by_user_id(
        &self,
        request: GrpcRequest<GetDsSessionBeanUserIdRequest>,
    ) -> GrpcResponse<GetDsSessionBeanUserIdResponse> {
        let conn = &self.conn;
        let user_id = request.into_inner().user_id;
        let ds_sessions: Vec<t_ds_session::Model> = t_ds_session::Entity::find()
            .filter(t_ds_session::Column::UserId.eq(user_id))
            .into_model()
            .all(conn)
            .await
            .map_err(|_| {
                tonic::Status::from_error(Box::<DolphinErrorInfo>::new(
                    AppStatus::LoginSessionFailed.into(),
                ))
            })?;

        Ok(tonic::Response::new(GetDsSessionBeanUserIdResponse {
            ds_session_beans: ds_sessions.into_iter().map(|v| v.into()).collect(),
        }))
    }
}
