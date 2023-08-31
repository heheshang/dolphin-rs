use super::service::DolphinRpcServer;
use proto::ds_project::ds_project_bean_service_server::DsProjectBeanService;


#[tonic::async_trait]
impl DsProjectBeanService for DolphinRpcServer {
    async fn list_ds_project_beans(
        &self,
        _req: tonic::Request<proto::ds_project::ListDsProjectBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_project::ListDsProjectBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_project_bean(
        &self,
        _req: tonic::Request<proto::ds_project::GetDsProjectBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProjectBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_project_bean(
        &self,
        _req: tonic::Request<proto::ds_project::CreateDsProjectBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProjectBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_project_bean(
        &self,
        _req: tonic::Request<proto::ds_project::UpdateDsProjectBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_project::DsProjectBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_project_bean(
        &self,
        _req: tonic::Request<proto::ds_project::DeleteDsProjectBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
