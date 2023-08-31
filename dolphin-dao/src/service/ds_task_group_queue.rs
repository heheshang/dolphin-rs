use super::service::DolphinRpcServer;
use proto::ds_task_group_queue::ds_task_group_queue_bean_service_server::DsTaskGroupQueueBeanService;


#[tonic::async_trait]
impl DsTaskGroupQueueBeanService for DolphinRpcServer {
    async fn list_ds_task_group_queue_beans(
        &self,
        _req: tonic::Request<proto::ds_task_group_queue::ListDsTaskGroupQueueBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_group_queue::ListDsTaskGroupQueueBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_group_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group_queue::GetDsTaskGroupQueueBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_group_queue::DsTaskGroupQueueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_task_group_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group_queue::CreateDsTaskGroupQueueBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_group_queue::DsTaskGroupQueueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_task_group_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group_queue::UpdateDsTaskGroupQueueBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_group_queue::DsTaskGroupQueueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_task_group_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group_queue::DeleteDsTaskGroupQueueBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
