use dolphin_common::core_results::results::GrpcRequest;
use proto::ds_alert_send_status::{
    ds_alert_send_status_bean_service_server::DsAlertSendStatusBeanService,
    CreateDsAlertSendStatusBeanRequest,
    DeleteDsAlertSendStatusBeanRequest,
    GetDsAlertSendStatusBeanRequest,
    ListDsAlertSendStatusBeansRequest,
    UpdateDsAlertSendStatusBeanRequest,
};
// use sea_orm::entity::prelude::*;

use super::service::DolphinRpcServer;


#[tonic::async_trait]
impl DsAlertSendStatusBeanService for DolphinRpcServer {
    async fn list_ds_alert_send_status_beans(
        &self,
        _req: GrpcRequest<ListDsAlertSendStatusBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::ListDsAlertSendStatusBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_send_status_bean(
        &self,
        _req: GrpcRequest<GetDsAlertSendStatusBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatusBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_alert_send_status_bean(
        &self,
        _req: GrpcRequest<CreateDsAlertSendStatusBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatusBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_alert_send_status_bean(
        &self,
        _req: GrpcRequest<UpdateDsAlertSendStatusBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert_send_status::DsAlertSendStatusBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_alert_send_status_bean(
        &self,
        _req: GrpcRequest<DeleteDsAlertSendStatusBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
