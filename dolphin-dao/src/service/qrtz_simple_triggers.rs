use super::service::DolphinRpcServer;
use proto::qrtz_simple_triggers::qrtz_simple_trigger_bean_service_server::QrtzSimpleTriggerBeanService;

#[tonic::async_trait]
impl QrtzSimpleTriggerBeanService for DolphinRpcServer {
    async fn list_qrtz_simple_trigger_beans(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::ListQrtzSimpleTriggerBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::ListQrtzSimpleTriggerBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_simple_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::GetQrtzSimpleTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_simple_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::CreateQrtzSimpleTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_simple_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::UpdateQrtzSimpleTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simple_triggers::QrtzSimpleTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_simple_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simple_triggers::DeleteQrtzSimpleTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
