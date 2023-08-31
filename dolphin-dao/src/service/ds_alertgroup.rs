use super::service::DolphinRpcServer;
use proto::ds_alertgroup::ds_alert_group_bean_service_server::DsAlertGroupBeanService;

#[tonic::async_trait]
impl DsAlertGroupBeanService for DolphinRpcServer {
    async fn list_ds_alert_group_beans(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::ListDsAlertGroupBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_alertgroup::ListDsAlertGroupBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_alert_group_bean(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::GetDsAlertGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_alert_group_bean(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::CreateDsAlertGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_alert_group_bean(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::UpdateDsAlertGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_alertgroup::DsAlertGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_alert_group_bean(
        &self,
        _req: tonic::Request<proto::ds_alertgroup::DeleteDsAlertGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
