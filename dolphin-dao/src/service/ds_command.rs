use super::service::DolphinRpcServer;
use proto::ds_command::ds_command_bean_service_server::DsCommandBeanService;


#[tonic::async_trait]
impl DsCommandBeanService for DolphinRpcServer {
    async fn list_ds_command_beans(
        &self,
        _req: tonic::Request<proto::ds_command::ListDsCommandBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_command::ListDsCommandBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_command_bean(
        &self,
        _req: tonic::Request<proto::ds_command::GetDsCommandBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommandBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_command_bean(
        &self,
        _req: tonic::Request<proto::ds_command::CreateDsCommandBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommandBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_command_bean(
        &self,
        _req: tonic::Request<proto::ds_command::UpdateDsCommandBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_command::DsCommandBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_command_bean(
        &self,
        _req: tonic::Request<proto::ds_command::DeleteDsCommandBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
