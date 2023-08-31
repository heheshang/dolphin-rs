use super::service::DolphinRpcServer;
use proto::ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_bean_service_server::DsDqRuleExecuteSqlBeanService;


#[tonic::async_trait]
impl DsDqRuleExecuteSqlBeanService for DolphinRpcServer {
    async fn list_ds_dq_rule_execute_sql_beans(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::ListDsDqRuleExecuteSqlBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::ListDsDqRuleExecuteSqlBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::GetDsDqRuleExecuteSqlBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::CreateDsDqRuleExecuteSqlBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::UpdateDsDqRuleExecuteSqlBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_execute_sql::DsDqRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_execute_sql::DeleteDsDqRuleExecuteSqlBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
