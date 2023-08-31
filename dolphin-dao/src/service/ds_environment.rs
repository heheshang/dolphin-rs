use super::service::DolphinRpcServer;
use proto::ds_environment::ds_environment_bean_service_server::DsEnvironmentBeanService;


#[tonic::async_trait]
impl DsEnvironmentBeanService for DolphinRpcServer {
    async fn list_ds_environment_beans(
        &self,
        _req: tonic::Request<proto::ds_environment::ListDsEnvironmentBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment::ListDsEnvironmentBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_environment_bean(
        &self,
        _req: tonic::Request<proto::ds_environment::GetDsEnvironmentBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironmentBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_environment_bean(
        &self,
        _req: tonic::Request<proto::ds_environment::CreateDsEnvironmentBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironmentBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_environment_bean(
        &self,
        _req: tonic::Request<proto::ds_environment::UpdateDsEnvironmentBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_environment::DsEnvironmentBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_environment_bean(
        &self,
        _req: tonic::Request<proto::ds_environment::DeleteDsEnvironmentBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
