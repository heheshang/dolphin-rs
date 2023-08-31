use super::service::DolphinRpcServer;
use proto::ds_version::ds_version_bean_service_server::DsVersionBeanService;

#[tonic::async_trait]
impl DsVersionBeanService for DolphinRpcServer {
    async fn list_ds_version_beans(
        &self,
        _req: tonic::Request<proto::ds_version::ListDsVersionBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_version::ListDsVersionBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_version_bean(
        &self,
        _req: tonic::Request<proto::ds_version::GetDsVersionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersionBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_version_bean(
        &self,
        _req: tonic::Request<proto::ds_version::CreateDsVersionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersionBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_version_bean(
        &self,
        _req: tonic::Request<proto::ds_version::UpdateDsVersionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_version::DsVersionBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_version_bean(
        &self,
        _req: tonic::Request<proto::ds_version::DeleteDsVersionBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
