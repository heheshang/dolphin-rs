use super::service::DolphinRpcServer;
use proto::ds_process_definition_log::ds_process_definition_log_bean_service_server::DsProcessDefinitionLogBeanService;

#[tonic::async_trait]
impl DsProcessDefinitionLogBeanService for DolphinRpcServer {
    async fn list_ds_process_definition_log_beans(
        &self,
        _req: tonic::Request<
            proto::ds_process_definition_log::ListDsProcessDefinitionLogBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition_log::ListDsProcessDefinitionLogBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_definition_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_definition_log::GetDsProcessDefinitionLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_definition_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_definition_log::CreateDsProcessDefinitionLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_definition_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_definition_log::UpdateDsProcessDefinitionLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition_log::DsProcessDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_definition_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_definition_log::DeleteDsProcessDefinitionLogBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
