use super::service::DolphinRpcServer;

use proto::ds_datasource::ds_datasource_bean_service_server::DsDatasourceBeanService;

#[tonic::async_trait]
impl DsDatasourceBeanService for DolphinRpcServer {
    async fn list_ds_datasource_beans(
        &self,
        _req: tonic::Request<proto::ds_datasource::ListDsDatasourceBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_datasource::ListDsDatasourceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_datasource_bean(
        &self,
        _req: tonic::Request<proto::ds_datasource::GetDsDatasourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_datasource_bean(
        &self,
        _req: tonic::Request<proto::ds_datasource::CreateDsDatasourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_datasource_bean(
        &self,
        _req: tonic::Request<proto::ds_datasource::UpdateDsDatasourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_datasource::DsDatasourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_datasource_bean(
        &self,
        _req: tonic::Request<proto::ds_datasource::DeleteDsDatasourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
