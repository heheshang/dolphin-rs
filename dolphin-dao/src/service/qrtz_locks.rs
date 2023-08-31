use super::service::DolphinRpcServer;
use proto::qrtz_locks::qrtz_locks_bean_service_server::QrtzLocksBeanService;

#[tonic::async_trait]
impl QrtzLocksBeanService for DolphinRpcServer {
    async fn list_qrtz_locks_beans(
        &self,
        _req: tonic::Request<proto::qrtz_locks::ListQrtzLocksBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_locks::ListQrtzLocksBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_locks_bean(
        &self,
        _req: tonic::Request<proto::qrtz_locks::GetQrtzLocksBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocksBean>, tonic::Status> {
        todo!()
    }

    async fn create_qrtz_locks_bean(
        &self,
        _req: tonic::Request<proto::qrtz_locks::CreateQrtzLocksBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocksBean>, tonic::Status> {
        todo!()
    }

    async fn update_qrtz_locks_bean(
        &self,
        _req: tonic::Request<proto::qrtz_locks::UpdateQrtzLocksBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_locks::QrtzLocksBean>, tonic::Status> {
        todo!()
    }

    async fn delete_qrtz_locks_bean(
        &self,
        _req: tonic::Request<proto::qrtz_locks::DeleteQrtzLocksBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
