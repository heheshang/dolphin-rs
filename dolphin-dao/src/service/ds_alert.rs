use super::service::DolphinRpcServer;
use proto::ds_alert::ds_alert_bean_service_server::DsAlertBeanService;
#[tonic::async_trait]
impl DsAlertBeanService for DolphinRpcServer {
    async fn list_ds_alert_beans(
        &self,
        _req: tonic::Request<proto::ds_alert::ListDsAlertBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alert::ListDsAlertBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_bean(
        &self,
        _req: tonic::Request<proto::ds_alert::GetDsAlertBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alert::DsAlertBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_alert_bean(
        &self,
        _req: tonic::Request<proto::ds_alert::CreateDsAlertBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alert::DsAlertBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_alert_bean(
        &self,
        _req: tonic::Request<proto::ds_alert::UpdateDsAlertBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alert::DsAlertBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_alert_bean(
        &self,
        _req: tonic::Request<proto::ds_alert::DeleteDsAlertBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
