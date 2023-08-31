use super::service::DolphinRpcServer;
use proto::ds_audit_log::ds_audit_log_bean_service_server::DsAuditLogBeanService;


#[tonic::async_trait]
impl DsAuditLogBeanService for DolphinRpcServer {
    async fn list_ds_audit_log_beans(
        &self,
        _req: tonic::Request<proto::ds_audit_log::ListDsAuditLogBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_audit_log::ListDsAuditLogBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_audit_log_bean(
        &self,
        _req: tonic::Request<proto::ds_audit_log::GetDsAuditLogBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLogBean>, tonic::Status>
    {
        todo!()
    }

    async fn create_ds_audit_log_bean(
        &self,
        _req: tonic::Request<proto::ds_audit_log::CreateDsAuditLogBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLogBean>, tonic::Status>
    {
        todo!()
    }

    async fn update_ds_audit_log_bean(
        &self,
        _req: tonic::Request<proto::ds_audit_log::UpdateDsAuditLogBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_audit_log::DsAuditLogBean>, tonic::Status>
    {
        todo!()
    }

    async fn delete_ds_audit_log_bean(
        &self,
        _req: tonic::Request<proto::ds_audit_log::DeleteDsAuditLogBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
