use super::service::DolphinRpcServer;
use proto::ds_task_instance::ds_task_instance_bean_service_server::DsTaskInstanceBeanService;

#[tonic::async_trait]
impl DsTaskInstanceBeanService for DolphinRpcServer {
    async fn list_ds_task_instance_beans(
        &self,
        _req: tonic::Request<proto::ds_task_instance::ListDsTaskInstanceBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_instance::ListDsTaskInstanceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_task_instance::GetDsTaskInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_instance::DsTaskInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_task_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_task_instance::CreateDsTaskInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_instance::DsTaskInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_task_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_task_instance::UpdateDsTaskInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_instance::DsTaskInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_task_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_task_instance::DeleteDsTaskInstanceBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
