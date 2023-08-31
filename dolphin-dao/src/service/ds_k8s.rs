use super::service::DolphinRpcServer;
use proto::ds_k8s::ds_k8s_bean_service_server::DsK8sBeanService;


#[tonic::async_trait]
impl DsK8sBeanService for DolphinRpcServer {
    async fn list_ds_k8s_beans(
        &self,
        _req: tonic::Request<proto::ds_k8s::ListDsK8sBeansRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s::ListDsK8sBeansResponse>, tonic::Status>
    {
        todo!()
    }

    async fn get_ds_k8s_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s::GetDsK8sBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s::DsK8sBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_k8s_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s::CreateDsK8sBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s::DsK8sBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_k8s_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s::UpdateDsK8sBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_k8s::DsK8sBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_k8s_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s::DeleteDsK8sBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
