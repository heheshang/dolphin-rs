use super::dao_service::DolphinRpcServer;
use proto::ds_process_definition::ds_process_definition_service_server::DsProcessDefinitionService;

#[tonic::async_trait]
impl DsProcessDefinitionService for DolphinRpcServer {
    async fn list_ds_process_definitions(
        &self,
        _req: tonic::Request<proto::ds_process_definition::ListDsProcessDefinitionsRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::ListDsProcessDefinitionsResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_definition(
        &self,
        _req: tonic::Request<proto::ds_process_definition::GetDsProcessDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_definition(
        &self,
        _req: tonic::Request<proto::ds_process_definition::CreateDsProcessDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_definition(
        &self,
        _req: tonic::Request<proto::ds_process_definition::UpdateDsProcessDefinitionRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_definition::DsProcessDefinition>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_definition(
        &self,
        _req: tonic::Request<proto::ds_process_definition::DeleteDsProcessDefinitionRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
