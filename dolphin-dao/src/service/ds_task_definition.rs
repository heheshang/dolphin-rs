use super::service::DolphinRpcServer;
use proto::ds_task_definition::ds_task_definition_bean_service_server::DsTaskDefinitionBeanService;

#[tonic::async_trait]
impl DsTaskDefinitionBeanService for DolphinRpcServer {
    async fn list_ds_task_definition_beans(
        &self,
        _req: tonic::Request<proto::ds_task_definition::ListDsTaskDefinitionBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::ListDsTaskDefinitionBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition::GetDsTaskDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_task_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition::CreateDsTaskDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_task_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition::UpdateDsTaskDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_definition::DsTaskDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_task_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_task_definition::DeleteDsTaskDefinitionBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
