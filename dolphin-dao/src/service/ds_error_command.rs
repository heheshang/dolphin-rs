use super::service::DolphinRpcServer;
use proto::ds_error_command::ds_error_command_bean_service_server::DsErrorCommandBeanService;


#[tonic::async_trait]
impl DsErrorCommandBeanService for DolphinRpcServer {
    async fn list_ds_error_command_beans(
        &self,
        _req: tonic::Request<proto::ds_error_command::ListDsErrorCommandBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_error_command::ListDsErrorCommandBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_error_command_bean(
        &self,
        _req: tonic::Request<proto::ds_error_command::GetDsErrorCommandBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_error_command::DsErrorCommandBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_error_command_bean(
        &self,
        _req: tonic::Request<proto::ds_error_command::CreateDsErrorCommandBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_error_command::DsErrorCommandBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_error_command_bean(
        &self,
        _req: tonic::Request<proto::ds_error_command::UpdateDsErrorCommandBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_error_command::DsErrorCommandBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_error_command_bean(
        &self,
        _req: tonic::Request<proto::ds_error_command::DeleteDsErrorCommandBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
