use super::service::DolphinRpcServer;
use proto::qrtz_cron_triggers::qrtz_cron_triggers_bean_service_server::QrtzCronTriggersBeanService;

#[tonic::async_trait]
impl QrtzCronTriggersBeanService for DolphinRpcServer {
    async fn list_qrtz_cron_triggers_beans(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::ListQrtzCronTriggersBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::ListQrtzCronTriggersBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_cron_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::GetQrtzCronTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_cron_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::CreateQrtzCronTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_cron_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::UpdateQrtzCronTriggersBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_cron_triggers::QrtzCronTriggersBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_cron_triggers_bean(
        &self,
        _req: tonic::Request<proto::qrtz_cron_triggers::DeleteQrtzCronTriggersBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
