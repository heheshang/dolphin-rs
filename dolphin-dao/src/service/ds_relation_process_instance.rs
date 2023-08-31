use super::service::DolphinRpcServer;
use proto::ds_relation_process_instance::ds_relation_process_instance_bean_service_server::DsRelationProcessInstanceBeanService;

#[tonic::async_trait]
impl DsRelationProcessInstanceBeanService for DolphinRpcServer {
    async fn list_ds_relation_process_instance_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_process_instance::ListDsRelationProcessInstanceBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_process_instance::ListDsRelationProcessInstanceBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_process_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_process_instance::GetDsRelationProcessInstanceBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_process_instance::DsRelationProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_process_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_process_instance::CreateDsRelationProcessInstanceBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_process_instance::DsRelationProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_process_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_process_instance::UpdateDsRelationProcessInstanceBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_process_instance::DsRelationProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_process_instance_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_process_instance::DeleteDsRelationProcessInstanceBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
