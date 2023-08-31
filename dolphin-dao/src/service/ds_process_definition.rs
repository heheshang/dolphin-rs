use super::service::DolphinRpcServer;
use proto::ds_process_definition::ds_process_definition_bean_service_server::DsProcessDefinitionBeanService;

#[tonic::async_trait]
impl DsProcessDefinitionBeanService for DolphinRpcServer {
    async fn list_ds_process_definition_beans(
        &self,
        _req: tonic::Request<proto::ds_process_definition::ListDsProcessDefinitionBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::ListDsProcessDefinitionBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_process_definition::GetDsProcessDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_process_definition::CreateDsProcessDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_process_definition::UpdateDsProcessDefinitionBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinitionBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_definition_bean(
        &self,
        _req: tonic::Request<proto::ds_process_definition::DeleteDsProcessDefinitionBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
