use super::service::DolphinRpcServer;
use proto::qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_bean_service_server::QrtzPausedTriggerGrpsBeanService;


#[tonic::async_trait]
impl QrtzPausedTriggerGrpsBeanService for DolphinRpcServer {
    async fn list_qrtz_paused_trigger_grps_beans(
        &self,
        _req: tonic::Request<
            proto::qrtz_paused_trigger_grps::ListQrtzPausedTriggerGrpsBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_paused_trigger_grps::ListQrtzPausedTriggerGrpsBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_paused_trigger_grps_bean(
        &self,
        _req: tonic::Request<proto::qrtz_paused_trigger_grps::GetQrtzPausedTriggerGrpsBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrpsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_paused_trigger_grps_bean(
        &self,
        _req: tonic::Request<
            proto::qrtz_paused_trigger_grps::CreateQrtzPausedTriggerGrpsBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrpsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_paused_trigger_grps_bean(
        &self,
        _req: tonic::Request<
            proto::qrtz_paused_trigger_grps::UpdateQrtzPausedTriggerGrpsBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_paused_trigger_grps::QrtzPausedTriggerGrpsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_paused_trigger_grps_bean(
        &self,
        _req: tonic::Request<
            proto::qrtz_paused_trigger_grps::DeleteQrtzPausedTriggerGrpsBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
