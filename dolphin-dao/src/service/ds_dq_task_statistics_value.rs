use super::service::DolphinRpcServer;
use proto::ds_dq_task_statistics_value::ds_dq_task_statistics_value_bean_service_server::DsDqTaskStatisticsValueBeanService;


#[tonic::async_trait]
impl DsDqTaskStatisticsValueBeanService for DolphinRpcServer {
    async fn list_ds_dq_task_statistics_value_beans(
        &self,
        _req: tonic::Request<
            proto::ds_dq_task_statistics_value::ListDsDqTaskStatisticsValueBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_dq_task_statistics_value::ListDsDqTaskStatisticsValueBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_task_statistics_value_bean(
        &self,
        _req: tonic::Request<
            proto::ds_dq_task_statistics_value::GetDsDqTaskStatisticsValueBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_task_statistics_value_bean(
        &self,
        _req: tonic::Request<
            proto::ds_dq_task_statistics_value::CreateDsDqTaskStatisticsValueBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_task_statistics_value_bean(
        &self,
        _req: tonic::Request<
            proto::ds_dq_task_statistics_value::UpdateDsDqTaskStatisticsValueBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_task_statistics_value::DsDqTaskStatisticsValueBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_task_statistics_value_bean(
        &self,
        _req: tonic::Request<
            proto::ds_dq_task_statistics_value::DeleteDsDqTaskStatisticsValueBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
