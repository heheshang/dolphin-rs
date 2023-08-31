use super::service::DolphinRpcServer;

use proto::ds_relation_namespace_user::ds_relation_namespace_user_bean_service_server::DsRelationNamespaceUserBeanService;


#[tonic::async_trait]
impl DsRelationNamespaceUserBeanService for DolphinRpcServer {
    async fn list_ds_relation_namespace_user_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::ListDsRelationNamespaceUserBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_namespace_user::ListDsRelationNamespaceUserBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_namespace_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::GetDsRelationNamespaceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_namespace_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::CreateDsRelationNamespaceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_namespace_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::UpdateDsRelationNamespaceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_namespace_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::DeleteDsRelationNamespaceUserBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
