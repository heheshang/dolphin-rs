use super::service::DolphinRpcServer;
use proto::ds_relation_datasource_user::ds_relation_datasource_user_bean_service_server::DsRelationDatasourceUserBeanService;

#[tonic::async_trait]
impl DsRelationDatasourceUserBeanService for DolphinRpcServer {
    async fn list_ds_relation_datasource_user_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_datasource_user::ListDsRelationDatasourceUserBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_datasource_user::ListDsRelationDatasourceUserBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_datasource_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_datasource_user::GetDsRelationDatasourceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_datasource_user::DsRelationDatasourceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_datasource_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_datasource_user::CreateDsRelationDatasourceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_datasource_user::DsRelationDatasourceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_datasource_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_datasource_user::UpdateDsRelationDatasourceUserBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_datasource_user::DsRelationDatasourceUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_datasource_user_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_datasource_user::DeleteDsRelationDatasourceUserBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
