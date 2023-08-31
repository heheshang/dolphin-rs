use super::service::DolphinRpcServer;
use proto::ds_queue::ds_queue_bean_service_server::DsQueueBeanService;


#[tonic::async_trait]
impl DsQueueBeanService for DolphinRpcServer {
    async fn list_ds_queue_beans(
        &self,
        _req: tonic::Request<proto::ds_queue::ListDsQueueBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_queue::ListDsQueueBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_queue::GetDsQueueBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueueBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_queue::CreateDsQueueBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueueBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_queue::UpdateDsQueueBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_queue::DsQueueBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_queue_bean(
        &self,
        _req: tonic::Request<proto::ds_queue::DeleteDsQueueBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
