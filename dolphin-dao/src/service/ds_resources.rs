use super::service::DolphinRpcServer;
use proto::ds_resources::ds_resource_bean_service_server::DsResourceBeanService;


#[tonic::async_trait]
impl DsResourceBeanService for DolphinRpcServer {
    async fn list_ds_resource_beans(
        &self,
        _req: tonic::Request<proto::ds_resources::ListDsResourceBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_resources::ListDsResourceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_resource_bean(
        &self,
        _req: tonic::Request<proto::ds_resources::GetDsResourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_resource_bean(
        &self,
        _req: tonic::Request<proto::ds_resources::CreateDsResourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_resource_bean(
        &self,
        _req: tonic::Request<proto::ds_resources::UpdateDsResourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_resources::DsResourceBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_resource_bean(
        &self,
        _req: tonic::Request<proto::ds_resources::DeleteDsResourceBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
