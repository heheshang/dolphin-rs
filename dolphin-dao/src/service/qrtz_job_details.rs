use super::dao_service::DolphinRpcServer;
use proto::qrtz_job_details::qrtz_job_details_service_server::QrtzJobDetailsService;

#[tonic::async_trait]
impl QrtzJobDetailsService for DolphinRpcServer {
    async fn list_qrtz_job_detailss(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::ListQrtzJobDetailssRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_job_details::ListQrtzJobDetailssResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_job_details(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::GetQrtzJobDetailsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_job_details::QrtzJobDetails>, tonic::Status>
    {
        todo!()
    }

    async fn create_qrtz_job_details(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::CreateQrtzJobDetailsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_job_details::QrtzJobDetails>, tonic::Status>
    {
        todo!()
    }

    async fn update_qrtz_job_details(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::UpdateQrtzJobDetailsRequest>,
    ) -> std::result::Result<tonic::Response<proto::qrtz_job_details::QrtzJobDetails>, tonic::Status>
    {
        todo!()
    }

    async fn delete_qrtz_job_details(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::DeleteQrtzJobDetailsRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
