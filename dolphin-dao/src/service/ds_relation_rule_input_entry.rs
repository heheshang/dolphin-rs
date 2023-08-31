use super::service::DolphinRpcServer;
use proto::ds_relation_rule_input_entry::ds_relation_rule_input_entry_bean_service_server::DsRelationRuleInputEntryBeanService;


#[tonic::async_trait]
impl DsRelationRuleInputEntryBeanService for DolphinRpcServer {
    async fn list_ds_relation_rule_input_entry_beans(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_input_entry::ListDsRelationRuleInputEntryBeansRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<
            proto::ds_relation_rule_input_entry::ListDsRelationRuleInputEntryBeansResponse,
        >,
        tonic::Status,
    > {
        todo!()
    }

    async fn get_ds_relation_rule_input_entry_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_input_entry::GetDsRelationRuleInputEntryBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_input_entry::DsRelationRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn create_ds_relation_rule_input_entry_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_input_entry::CreateDsRelationRuleInputEntryBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_input_entry::DsRelationRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn update_ds_relation_rule_input_entry_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_input_entry::UpdateDsRelationRuleInputEntryBeanRequest,
        >,
    ) -> std::result::Result<
        tonic::Response<proto::ds_relation_rule_input_entry::DsRelationRuleInputEntryBean>,
        tonic::Status,
    > {
        todo!()
    }

    async fn delete_ds_relation_rule_input_entry_bean(
        &self,
        _req: tonic::Request<
            proto::ds_relation_rule_input_entry::DeleteDsRelationRuleInputEntryBeanRequest,
        >,
    ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
        todo!()
    }
}
