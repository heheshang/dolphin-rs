use super::service::DolphinRpcServer;
use proto::qrtz_blob_triggers::qrtz_blob_trigger_bean_service_server::QrtzBlobTriggerBeanService;

#[tonic::async_trait]
impl QrtzBlobTriggerBeanService for DolphinRpcServer {
    async fn list_qrtz_blob_trigger_beans(
        &self,
        _req: tonic::Request<proto::qrtz_blob_triggers::ListQrtzBlobTriggerBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_blob_triggers::ListQrtzBlobTriggerBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_blob_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_blob_triggers::GetQrtzBlobTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_blob_triggers::QrtzBlobTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_blob_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_blob_triggers::CreateQrtzBlobTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_blob_triggers::QrtzBlobTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_blob_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_blob_triggers::UpdateQrtzBlobTriggerBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_blob_triggers::QrtzBlobTriggerBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_blob_trigger_bean(
        &self,
        _req: tonic::Request<proto::qrtz_blob_triggers::DeleteQrtzBlobTriggerBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
