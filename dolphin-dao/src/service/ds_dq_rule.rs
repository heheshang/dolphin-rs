use super::service::DolphinRpcServer;
use proto::ds_dq_rule::ds_dq_rule_bean_service_server::DsDqRuleBeanService;


#[tonic::async_trait]
impl DsDqRuleBeanService for DolphinRpcServer {
    async fn list_ds_dq_rule_beans(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::ListDsDqRuleBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule::ListDsDqRuleBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_rule_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::GetDsDqRuleBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRuleBean>, tonic::Status> {
        todo!()
    }

    async fn create_ds_dq_rule_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::CreateDsDqRuleBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRuleBean>, tonic::Status> {
        todo!()
    }

    async fn update_ds_dq_rule_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::UpdateDsDqRuleBeanRequest>,
    ) -> std::result::Result<tonic::Response<proto::ds_dq_rule::DsDqRuleBean>, tonic::Status> {
        todo!()
    }

    async fn delete_ds_dq_rule_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule::DeleteDsDqRuleBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
