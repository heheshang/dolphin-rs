use super::service::DolphinRpcServer;
use proto::qrtz_fired_triggers::qrtz_fired_triggers_bean_service_server::QrtzFiredTriggersBeanService;

#[tonic::async_trait]
impl QrtzFiredTriggersBeanService for DolphinRpcServer {
    async fn list_qrtz_fired_triggers_beans(
        &self,
        _req: tonic::Request<proto::qrtz_fired_triggers::ListQrtzFiredTriggersBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_fired_triggers::ListQrtzFiredTriggersBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_fired_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_fired_triggers::GetQrtzFiredTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_fired_triggers::QrtzFiredTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_fired_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_fired_triggers::CreateQrtzFiredTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_fired_triggers::QrtzFiredTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_fired_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_fired_triggers::UpdateQrtzFiredTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_fired_triggers::QrtzFiredTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_fired_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_fired_triggers::DeleteQrtzFiredTriggersBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
