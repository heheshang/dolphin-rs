use super::service::DolphinRpcServer;
use proto::ds_dq_execute_result::ds_dq_execute_result_bean_service_server::DsDqExecuteResultBeanService;


#[tonic::async_trait]
impl DsDqExecuteResultBeanService for DolphinRpcServer {
    async fn list_ds_dq_execute_result_beans(
        &self,
        _req: tonic::Request<proto::ds_dq_execute_result::ListDsDqExecuteResultBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_execute_result::ListDsDqExecuteResultBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_execute_result_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_execute_result::GetDsDqExecuteResultBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_execute_result::DsDqExecuteResultBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_execute_result_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_execute_result::CreateDsDqExecuteResultBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_execute_result::DsDqExecuteResultBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_execute_result_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_execute_result::UpdateDsDqExecuteResultBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_execute_result::DsDqExecuteResultBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_execute_result_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_execute_result::DeleteDsDqExecuteResultBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
