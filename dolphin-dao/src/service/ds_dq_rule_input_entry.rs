use super::service::DolphinRpcServer;
use proto::ds_dq_rule_input_entry::ds_dq_rule_input_entry_bean_service_server::DsDqRuleInputEntryBeanService;


#[tonic::async_trait]
impl DsDqRuleInputEntryBeanService for DolphinRpcServer {
    async fn list_ds_dq_rule_input_entry_beans(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::ListDsDqRuleInputEntryBeansRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::ListDsDqRuleInputEntryBeansResponse>,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_dq_rule_input_entry_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::GetDsDqRuleInputEntryBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_dq_rule_input_entry_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::CreateDsDqRuleInputEntryBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_dq_rule_input_entry_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::UpdateDsDqRuleInputEntryBeanRequest>,
    ) -> std::result::Result<
        tonic::Response<proto::ds_dq_rule_input_entry::DsDqRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_dq_rule_input_entry_bean(
        &self,
        _req: tonic::Request<proto::ds_dq_rule_input_entry::DeleteDsDqRuleInputEntryBeanRequest>,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
