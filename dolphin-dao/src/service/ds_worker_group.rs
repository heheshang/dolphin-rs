use super::service::DolphinRpcServer;
use proto::ds_worker_group::ds_worker_group_bean_service_server::DsWorkerGroupBeanService;

#[tonic::async_trait]
impl DsWorkerGroupBeanService for DolphinRpcServer {
    async fn list_ds_worker_group_beans(
        &self,
        _req: tonic::Request<proto::ds_worker_group::ListDsWorkerGroupBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_worker_group::ListDsWorkerGroupBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_worker_group_bean(
        &self,
        _req: tonic::Request<proto::ds_worker_group::GetDsWorkerGroupBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_worker_group::DsWorkerGroupBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_worker_group_bean(
        &self,
        _req: tonic::Request<proto::ds_worker_group::CreateDsWorkerGroupBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_worker_group::DsWorkerGroupBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_worker_group_bean(
        &self,
        _req: tonic::Request<proto::ds_worker_group::UpdateDsWorkerGroupBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_worker_group::DsWorkerGroupBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_worker_group_bean(
        &self,
        _req: tonic::Request<proto::ds_worker_group::DeleteDsWorkerGroupBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
