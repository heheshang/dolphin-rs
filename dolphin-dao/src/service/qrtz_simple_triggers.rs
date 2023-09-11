use super::dao_service::DolphinRpcServer;
use proto::qrtz_simple_triggers::qrtz_simple_trigger_service_server::QrtzSimpleTriggerService;

#[tonic::async_trait]
impl QrtzSimpleTriggerService for DolphinRpcServer {
    async fn list_qrtz_simple_triggers(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::ListQrtzSimpleTriggersRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::ListQrtzSimpleTriggersResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_simple_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::GetQrtzSimpleTriggerRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTrigger>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_simple_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::CreateQrtzSimpleTriggerRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTrigger>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_simple_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::UpdateQrtzSimpleTriggerRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTrigger>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_simple_trigger(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::DeleteQrtzSimpleTriggerRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
