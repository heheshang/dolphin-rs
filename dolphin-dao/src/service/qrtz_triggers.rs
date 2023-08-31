use super::service::DolphinRpcServer;

use proto::qrtz_triggers::qrtz_trigger_bean_service_server::QrtzTriggerBeanService;
#[tonic::async_trait]
impl QrtzTriggerBeanService for DolphinRpcServer {
    async fn list_qrtz_trigger_beans(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::ListQrtzTriggerBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_triggers::ListQrtzTriggerBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::GetQrtzTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTriggerBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_qrtz_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::CreateQrtzTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTriggerBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_qrtz_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::UpdateQrtzTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_triggers::QrtzTriggerBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_qrtz_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_triggers::DeleteQrtzTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
