use super::service::DolphinRpcServer;
use proto::qrtz_simprop_triggers::qrtz_simprop_trigger_bean_service_server::QrtzSimpropTriggerBeanService;

#[tonic::async_trait]
impl QrtzSimpropTriggerBeanService for DolphinRpcServer {
    async fn list_qrtz_simprop_trigger_beans(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::ListQrtzSimpropTriggerBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simprop_triggers::ListQrtzSimpropTriggerBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_simprop_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::GetQrtzSimpropTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_simprop_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::CreateQrtzSimpropTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_simprop_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::UpdateQrtzSimpropTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_simprop_triggers::QrtzSimpropTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_simprop_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_simprop_triggers::DeleteQrtzSimpropTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
