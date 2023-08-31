use super::service::DolphinRpcServer;
use proto::qrtz_job_details::qrtz_job_details_bean_service_server::QrtzJobDetailsBeanService;

#[tonic::async_trait]
impl QrtzJobDetailsBeanService for DolphinRpcServer {
    async fn list_qrtz_job_details_beans(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::ListQrtzJobDetailsBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_job_details::ListQrtzJobDetailsBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_qrtz_job_details_bean(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::GetQrtzJobDetailsBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_job_details::QrtzJobDetailsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_qrtz_job_details_bean(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::CreateQrtzJobDetailsBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_job_details::QrtzJobDetailsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_qrtz_job_details_bean(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::UpdateQrtzJobDetailsBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::qrtz_job_details::QrtzJobDetailsBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_qrtz_job_details_bean(
        &self,
        _req: tonic::Request<proto::qrtz_job_details::DeleteQrtzJobDetailsBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
