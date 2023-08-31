use super::service::DolphinRpcServer;
use proto::ds_relation_udfs_user::ds_relation_udfs_user_bean_service_server::DsRelationUdfsUserBeanService;

#[tonic::async_trait]
impl DsRelationUdfsUserBeanService for DolphinRpcServer {
    async fn list_ds_relation_udfs_user_beans(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::ListDsRelationUdfsUserBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::ListDsRelationUdfsUserBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_udfs_user_bean(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::GetDsRelationUdfsUserBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_udfs_user_bean(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::CreateDsRelationUdfsUserBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_udfs_user_bean(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::UpdateDsRelationUdfsUserBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_udfs_user::DsRelationUdfsUserBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_udfs_user_bean(
        &self,
        _req: tonic::Request<proto::ds_relation_udfs_user::DeleteDsRelationUdfsUserBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
