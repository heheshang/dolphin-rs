use super::service::DolphinRpcServer;
use proto::ds_schedules::ds_schedules_bean_service_server::DsSchedulesBeanService;


#[tonic::async_trait]
impl DsSchedulesBeanService for DolphinRpcServer {
    async fn list_ds_schedules_beans(
        &self,
        _req: tonic::Request<proto::ds_schedules::ListDsSchedulesBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_schedules::ListDsSchedulesBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_schedules_bean(
        &self,
        _req: tonic::Request<proto::ds_schedules::GetDsSchedulesBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_schedules::DsSchedulesBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_schedules_bean(
        &self,
        _req: tonic::Request<proto::ds_schedules::CreateDsSchedulesBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_schedules::DsSchedulesBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_schedules_bean(
        &self,
        _req: tonic::Request<proto::ds_schedules::UpdateDsSchedulesBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_schedules::DsSchedulesBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_schedules_bean(
        &self,
        _req: tonic::Request<proto::ds_schedules::DeleteDsSchedulesBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
