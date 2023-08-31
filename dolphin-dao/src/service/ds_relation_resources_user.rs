use super::service::DolphinRpcServer;
use proto::ds_relation_resources_user::ds_relation_resources_user_bean_service_server::DsRelationResourcesUserBeanService;


#[tonic::async_trait]
impl DsRelationResourcesUserBeanService for DolphinRpcServer {
    async fn list_ds_relation_resources_user_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::ListDsRelationResourcesUserBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_resources_user::ListDsRelationResourcesUserBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_resources_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::GetDsRelationResourcesUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_resources_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::CreateDsRelationResourcesUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_resources_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::UpdateDsRelationResourcesUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_resources_user::DsRelationResourcesUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_resources_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_resources_user::DeleteDsRelationResourcesUserBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
