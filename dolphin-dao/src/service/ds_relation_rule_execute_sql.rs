use super::service::DolphinRpcServer;
use proto::ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_bean_service_server::DsRelationRuleExecuteSqlBeanService;

#[tonic::async_trait]
impl DsRelationRuleExecuteSqlBeanService for DolphinRpcServer {
    async fn list_ds_relation_rule_execute_sql_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_execute_sql::ListDsRelationRuleExecuteSqlBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_rule_execute_sql::ListDsRelationRuleExecuteSqlBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_execute_sql::GetDsRelationRuleExecuteSqlBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_execute_sql::CreateDsRelationRuleExecuteSqlBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_execute_sql::UpdateDsRelationRuleExecuteSqlBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_execute_sql::DsRelationRuleExecuteSqlBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_rule_execute_sql_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_execute_sql::DeleteDsRelationRuleExecuteSqlBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
