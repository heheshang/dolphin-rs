use super::service::DolphinRpcServer;
use proto::qrtz_calendars::qrtz_calendar_bean_service_server::QrtzCalendarBeanService;


#[tonic::async_trait]
impl QrtzCalendarBeanService for DolphinRpcServer {
    async fn list_qrtz_calendar_beans(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::ListQrtzCalendarBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_calendars::ListQrtzCalendarBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_calendar_bean(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::GetQrtzCalendarBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendarBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_qrtz_calendar_bean(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::CreateQrtzCalendarBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendarBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_qrtz_calendar_bean(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::UpdateQrtzCalendarBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_calendars::QrtzCalendarBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_qrtz_calendar_bean(
        &self,
        _req: tonic::Request<proto::qrtz_calendars::DeleteQrtzCalendarBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
