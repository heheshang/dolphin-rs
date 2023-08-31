use super::service::DolphinRpcServer;
use proto::ds_process_instance::ds_process_instance_bean_service_server::DsProcessInstanceBeanService;

#[tonic::async_trait]
impl DsProcessInstanceBeanService for DolphinRpcServer {
    async fn list_ds_process_instance_beans(
        &self,
        _req: tonic::Request<proto::ds_process_instance::ListDsProcessInstanceBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_instance::ListDsProcessInstanceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_process_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_process_instance::GetDsProcessInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_instance::DsProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_process_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_process_instance::CreateDsProcessInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_instance::DsProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_process_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_process_instance::UpdateDsProcessInstanceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_process_instance::DsProcessInstanceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_process_instance_bean(
        &self,
        _req: tonic::Request<proto::ds_process_instance::DeleteDsProcessInstanceBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
