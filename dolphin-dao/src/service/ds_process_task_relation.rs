use super::service::DolphinRpcServer;
use proto::ds_process_task_relation::ds_process_task_relation_bean_service_server::DsProcessTaskRelationBeanService;

#[tonic::async_trait]
impl DsProcessTaskRelationBeanService for DolphinRpcServer {
    async fn list_ds_process_task_relation_beans(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation::ListDsProcessTaskRelationBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation::ListDsProcessTaskRelationBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_task_relation_bean(
        &self,
        _req: tonic::Request<proto::ds_process_task_relation::GetDsProcessTaskRelationBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelationBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_task_relation_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation::CreateDsProcessTaskRelationBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelationBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_task_relation_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation::UpdateDsProcessTaskRelationBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation::DsProcessTaskRelationBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_task_relation_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation::DeleteDsProcessTaskRelationBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
