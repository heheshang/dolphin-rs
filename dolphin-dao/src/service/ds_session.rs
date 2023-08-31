use super::service::DolphinRpcServer;
use proto::ds_session::ds_session_bean_service_server::DsSessionBeanService;

#[tonic::async_trait]
impl DsSessionBeanService for DolphinRpcServer {
    async fn list_ds_session_beans(
        &self,
        _req: tonic::Request<proto::ds_session::ListDsSessionBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_session::ListDsSessionBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::GetDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::CreateDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::UpdateDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_session::DsSessionBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_session_bean(
        &self,
        _req: tonic::Request<proto::ds_session::DeleteDsSessionBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
