use super::service::DolphinRpcServer;
use proto::ds_dq_comparison_type::ds_dq_comparison_type_bean_service_server::DsDqComparisonTypeBeanService;

#[tonic::async_trait]
impl DsDqComparisonTypeBeanService for DolphinRpcServer {
    async fn list_ds_dq_comparison_type_beans(
        &self,
        _req: tonic::Request<proto::ds_dq_comparison_type::ListDsDqComparisonTypeBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_comparison_type::ListDsDqComparisonTypeBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_comparison_type_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_comparison_type::GetDsDqComparisonTypeBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_comparison_type::DsDqComparisonTypeBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_comparison_type_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_comparison_type::CreateDsDqComparisonTypeBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_comparison_type::DsDqComparisonTypeBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_comparison_type_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_comparison_type::UpdateDsDqComparisonTypeBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_comparison_type::DsDqComparisonTypeBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_comparison_type_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_comparison_type::DeleteDsDqComparisonTypeBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
