use super::service::DolphinRpcServer;
use proto::ds_tenant::ds_tenant_bean_service_server::DsTenantBeanService;
#[tonic::async_trait]
impl DsTenantBeanService for DolphinRpcServer {
    async fn list_ds_tenant_beans(
        &self,
        _req: tonic::Request<proto::ds_tenant::ListDsTenantBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_tenant::ListDsTenantBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_tenant_bean(
        &self,
        _req: tonic::Request<proto::ds_tenant::GetDsTenantBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenantBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_tenant_bean(
        &self,
        _req: tonic::Request<proto::ds_tenant::CreateDsTenantBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenantBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_tenant_bean(
        &self,
        _req: tonic::Request<proto::ds_tenant::UpdateDsTenantBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_tenant::DsTenantBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_tenant_bean(
        &self,
        _req: tonic::Request<proto::ds_tenant::DeleteDsTenantBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
