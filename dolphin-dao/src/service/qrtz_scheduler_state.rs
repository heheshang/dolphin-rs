use super::service::DolphinRpcServer;
use proto::qrtz_scheduler_state::qrtz_scheduler_state_bean_service_server::QrtzSchedulerStateBeanService;

#[tonic::async_trait]
impl QrtzSchedulerStateBeanService for DolphinRpcServer {
    async fn list_qrtz_scheduler_state_beans(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::ListQrtzSchedulerStateBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_scheduler_state::ListQrtzSchedulerStateBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_scheduler_state_bean(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::GetQrtzSchedulerStateBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerStateBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_scheduler_state_bean(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::CreateQrtzSchedulerStateBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerStateBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_scheduler_state_bean(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::UpdateQrtzSchedulerStateBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_scheduler_state::QrtzSchedulerStateBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_scheduler_state_bean(
        &self,
        _req: tonic::Request<proto::qrtz_scheduler_state::DeleteQrtzSchedulerStateBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
