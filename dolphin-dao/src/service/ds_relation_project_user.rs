use super::service::DolphinRpcServer;
use proto::ds_relation_project_user::ds_relation_project_user_bean_service_server::DsRelationProjectUserBeanService;


#[tonic::async_trait]
impl DsRelationProjectUserBeanService for DolphinRpcServer {
    async fn list_ds_relation_project_user_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_project_user::ListDsRelationProjectUserBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::ListDsRelationProjectUserBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_project_user_bean(
        &self,
        _req: tonic::Request<proto::ds_relation_project_user::GetDsRelationProjectUserBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_project_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_project_user::CreateDsRelationProjectUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_project_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_project_user::UpdateDsRelationProjectUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_project_user::DsRelationProjectUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_project_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_project_user::DeleteDsRelationProjectUserBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
