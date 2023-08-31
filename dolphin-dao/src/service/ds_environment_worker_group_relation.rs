use super::service::DolphinRpcServer;
use proto::ds_environment_worker_group_relation::ds_environment_worker_group_relation_bean_service_server::DsEnvironmentWorkerGroupRelationBeanService;


#[tonic::async_trait]
impl DsEnvironmentWorkerGroupRelationBeanService for DolphinRpcServer {
    async fn list_ds_environment_worker_group_relation_beans(
        &self,
       _req: tonic::Request<proto::ds_environment_worker_group_relation::ListDsEnvironmentWorkerGroupRelationBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_environment_worker_group_relation::ListDsEnvironmentWorkerGroupRelationBeansResponse>,
        tonic::Status,
    >{
        todo!()
    }

    async fn get_ds_environment_worker_group_relation_bean(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::GetDsEnvironmentWorkerGroupRelationBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelationBean,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_environment_worker_group_relation_bean(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::CreateDsEnvironmentWorkerGroupRelationBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelationBean,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_environment_worker_group_relation_bean(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::UpdateDsEnvironmentWorkerGroupRelationBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelationBean,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_environment_worker_group_relation_bean(
        &self,
        _req: tonic::Request<proto::ds_environment_worker_group_relation::DeleteDsEnvironmentWorkerGroupRelationBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
