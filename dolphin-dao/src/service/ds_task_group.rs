use super::service::DolphinRpcServer;
use proto::ds_task_group::ds_task_group_bean_service_server::DsTaskGroupBeanService;

#[tonic::async_trait]
impl DsTaskGroupBeanService for DolphinRpcServer {
    async fn list_ds_task_group_beans(
        &self,
        _req: tonic::Request<proto::ds_task_group::ListDsTaskGroupBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_task_group::ListDsTaskGroupBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_task_group_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group::GetDsTaskGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_task_group_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group::CreateDsTaskGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_task_group_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group::UpdateDsTaskGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_task_group::DsTaskGroupBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_task_group_bean(
        &self,
        _req: tonic::Request<proto::ds_task_group::DeleteDsTaskGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
