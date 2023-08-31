use super::service::DolphinRpcServer;
use proto::ds_k8s_namespace::ds_k8s_namespace_bean_service_server::DsK8sNamespaceBeanService;
#[tonic::async_trait]
impl DsK8sNamespaceBeanService for DolphinRpcServer {
    async fn list_ds_k8s_namespace_beans(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::ListDsK8sNamespaceBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_k8s_namespace::ListDsK8sNamespaceBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_k8s_namespace_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::GetDsK8sNamespaceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_k8s_namespace::DsK8sNamespaceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_k8s_namespace_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::CreateDsK8sNamespaceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_k8s_namespace::DsK8sNamespaceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_k8s_namespace_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::UpdateDsK8sNamespaceBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_k8s_namespace::DsK8sNamespaceBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_k8s_namespace_bean(
        &self,
        _req: tonic::Request<proto::ds_k8s_namespace::DeleteDsK8sNamespaceBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
