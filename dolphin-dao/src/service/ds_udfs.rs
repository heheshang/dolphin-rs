use super::dao_service::DolphinRpcServer;
use proto::ds_udfs::ds_udfs_service_server::DsUdfsService;
#[tonic::async_trait]
impl DsUdfsService for DolphinRpcServer {
    async fn list_ds_udfss(
        &self,
        _req: tonic::Request<proto::ds_udfs::ListDsUdfssRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::ListDsUdfssResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_udfs(
        &self,
        _req: tonic::Request<proto::ds_udfs::GetDsUdfsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfs>, tonic::Status> {
        todo!()
    }

    async fn create_ds_udfs(
        &self,
        _req: tonic::Request<proto::ds_udfs::CreateDsUdfsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfs>, tonic::Status> {
        todo!()
    }

    async fn update_ds_udfs(
        &self,
        _req: tonic::Request<proto::ds_udfs::UpdateDsUdfsRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_udfs::DsUdfs>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_udfs(
        &self,
        _req: tonic::Request<proto::ds_udfs::DeleteDsUdfsRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
