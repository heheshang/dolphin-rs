use super::dao_service::DolphinRpcServer;

use proto::ds_relation_namespace_user::ds_relation_namespace_user_service_server::DsRelationNamespaceUserService;


#[tonic::async_trait]
impl DsRelationNamespaceUserService for DolphinRpcServer {
    async fn list_ds_relation_namespace_users(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::ListDsRelationNamespaceUsersRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::ListDsRelationNamespaceUsersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_namespace_user(
        &self,
        _req: tonic::Request<proto::ds_relation_namespace_user::GetDsRelationNamespaceUserRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_namespace_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::CreateDsRelationNamespaceUserRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_namespace_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::UpdateDsRelationNamespaceUserRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_namespace_user::DsRelationNamespaceUser>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_namespace_user(
        &self,
        _req: tonic::Request<
            proto::ds_relation_namespace_user::DeleteDsRelationNamespaceUserRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
