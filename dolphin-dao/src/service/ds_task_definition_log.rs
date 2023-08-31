use super::service::DolphinRpcServer;
use proto::ds_task_definition_log::ds_task_definition_log_bean_service_server::DsTaskDefinitionLogBeanService;


#[tonic::async_trait]
impl DsTaskDefinitionLogBeanService for DolphinRpcServer {
    async fn list_ds_task_definition_log_beans(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::ListDsTaskDefinitionLogBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition_log::ListDsTaskDefinitionLogBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_definition_log_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::GetDsTaskDefinitionLogBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_task_definition_log_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::CreateDsTaskDefinitionLogBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_task_definition_log_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::UpdateDsTaskDefinitionLogBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition_log::DsTaskDefinitionLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_task_definition_log_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition_log::DeleteDsTaskDefinitionLogBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
