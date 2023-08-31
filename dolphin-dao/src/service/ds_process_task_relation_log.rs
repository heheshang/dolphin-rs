use super::service::DolphinRpcServer;
use proto::ds_process_task_relation_log::ds_process_task_relation_log_bean_service_server::DsProcessTaskRelationLogBeanService;


#[tonic::async_trait]
impl DsProcessTaskRelationLogBeanService for DolphinRpcServer {
    async fn list_ds_process_task_relation_log_beans(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation_log::ListDsProcessTaskRelationLogBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_process_task_relation_log::ListDsProcessTaskRelationLogBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_task_relation_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation_log::GetDsProcessTaskRelationLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation_log::DsProcessTaskRelationLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_task_relation_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation_log::CreateDsProcessTaskRelationLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation_log::DsProcessTaskRelationLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_task_relation_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation_log::UpdateDsProcessTaskRelationLogBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_task_relation_log::DsProcessTaskRelationLogBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_task_relation_log_bean(
        &self,
        _req: tonic::Request<
            proto::ds_process_task_relation_log::DeleteDsProcessTaskRelationLogBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
