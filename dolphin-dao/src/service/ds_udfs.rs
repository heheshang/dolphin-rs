use super::service::DolphinRpcServer;
use proto::ds_udfs::ds_udfs_bean_service_server::DsUdfsBeanService;
#[tonic::async_trait]
impl DsUdfsBeanService for DolphinRpcServer {
    async fn list_ds_udfs_beans(
        &self,
        _req: tonic::Request<proto::ds_udfs::ListDsUdfsBeansRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::ListDsUdfsBeansResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_udfs_bean(
        &self,
        _req: tonic::Request<proto::ds_udfs::GetDsUdfsBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfsBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_udfs_bean(
        &self,
        _req: tonic::Request<proto::ds_udfs::CreateDsUdfsBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfsBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_udfs_bean(
        &self,
        _req: tonic::Request<proto::ds_udfs::UpdateDsUdfsBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfsBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_udfs_bean(
        &self,
        _req: tonic::Request<proto::ds_udfs::DeleteDsUdfsBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
