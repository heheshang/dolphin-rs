use proto::{
ds_access_token::ds_access_token_bean_service_server::DsAccessTokenBeanServiceServer,
ds_alert::ds_alert_bean_service_server::DsAlertBeanServiceServer,
ds_alert_plugin_instance::ds_alert_plugin_instance_bean_service_server::DsAlertPluginInstanceBeanServiceServer,
ds_alert_send_status::ds_alert_send_status_bean_service_server::DsAlertSendStatusBeanServiceServer,
ds_alertgroup::ds_alert_group_bean_service_server::DsAlertGroupBeanServiceServer,
ds_audit_log::ds_audit_log_bean_service_server::DsAuditLogBeanServiceServer,
ds_command::ds_command_bean_service_server::DsCommandBeanServiceServer,
ds_datasource::ds_datasource_bean_service_server::DsDatasourceBeanServiceServer,
ds_dq_comparison_type::ds_dq_comparison_type_bean_service_server::DsDqComparisonTypeBeanServiceServer,
ds_dq_execute_result::ds_dq_execute_result_bean_service_server::DsDqExecuteResultBeanServiceServer,
ds_dq_rule::ds_dq_rule_bean_service_server::DsDqRuleBeanServiceServer,
ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_bean_service_server::DsDqRuleExecuteSqlBeanServiceServer,
ds_dq_rule_input_entry::ds_dq_rule_input_entry_bean_service_server::DsDqRuleInputEntryBeanServiceServer,
ds_dq_task_statistics_value::ds_dq_task_statistics_value_bean_service_server::DsDqTaskStatisticsValueBeanServiceServer,
ds_environment::ds_environment_bean_service_server::DsEnvironmentBeanServiceServer,
ds_environment_worker_group_relation::ds_environment_worker_group_relation_bean_service_server::DsEnvironmentWorkerGroupRelationBeanServiceServer,
ds_error_command::ds_error_command_bean_service_server::DsErrorCommandBeanServiceServer,
ds_k8s::ds_k8s_bean_service_server::DsK8sBeanServiceServer,
ds_k8s_namespace::ds_k8s_namespace_bean_service_server::DsK8sNamespaceBeanServiceServer,
ds_plugin_define::ds_plugin_define_bean_service_server::DsPluginDefineBeanServiceServer,
ds_process_definition::ds_process_definition_bean_service_server::DsProcessDefinitionBeanServiceServer,
ds_process_definition_log::ds_process_definition_log_bean_service_server::DsProcessDefinitionLogBeanServiceServer,
ds_process_instance::ds_process_instance_bean_service_server::DsProcessInstanceBeanServiceServer,
ds_process_task_relation::ds_process_task_relation_bean_service_server::DsProcessTaskRelationBeanServiceServer,
ds_process_task_relation_log::ds_process_task_relation_log_bean_service_server::DsProcessTaskRelationLogBeanServiceServer,
ds_project::ds_project_bean_service_server::DsProjectBeanServiceServer,
ds_queue::ds_queue_bean_service_server::DsQueueBeanServiceServer,
ds_relation_datasource_user::ds_relation_datasource_user_bean_service_server::DsRelationDatasourceUserBeanServiceServer,
ds_relation_namespace_user::ds_relation_namespace_user_bean_service_server::DsRelationNamespaceUserBeanServiceServer,
ds_relation_process_instance::ds_relation_process_instance_bean_service_server::DsRelationProcessInstanceBeanServiceServer,
ds_relation_project_user::ds_relation_project_user_bean_service_server::DsRelationProjectUserBeanServiceServer,
ds_relation_resources_user::ds_relation_resources_user_bean_service_server::DsRelationResourcesUserBeanServiceServer,
ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_bean_service_server::DsRelationRuleExecuteSqlBeanServiceServer,
ds_relation_rule_input_entry::ds_relation_rule_input_entry_bean_service_server::DsRelationRuleInputEntryBeanServiceServer,
ds_relation_udfs_user::ds_relation_udfs_user_bean_service_server::DsRelationUdfsUserBeanServiceServer,
ds_resources::ds_resource_bean_service_server::DsResourceBeanServiceServer,
ds_schedules::ds_schedules_bean_service_server::DsSchedulesBeanServiceServer,
ds_session::ds_session_bean_service_server::DsSessionBeanServiceServer,
ds_task_definition::ds_task_definition_bean_service_server::DsTaskDefinitionBeanServiceServer,
ds_task_definition_log::ds_task_definition_log_bean_service_server::DsTaskDefinitionLogBeanServiceServer,
ds_task_group::ds_task_group_bean_service_server::DsTaskGroupBeanServiceServer,
ds_task_group_queue::ds_task_group_queue_bean_service_server::DsTaskGroupQueueBeanServiceServer,
ds_task_instance::ds_task_instance_bean_service_server::DsTaskInstanceBeanServiceServer,
ds_tenant::ds_tenant_bean_service_server::DsTenantBeanServiceServer,
ds_udfs::ds_udfs_bean_service_server::DsUdfsBeanServiceServer,
ds_user::ds_user_bean_service_server::DsUserBeanServiceServer,
ds_version::ds_version_bean_service_server::DsVersionBeanServiceServer,
ds_worker_group::ds_worker_group_bean_service_server::DsWorkerGroupBeanServiceServer,
qrtz_blob_triggers::qrtz_blob_trigger_bean_service_server::QrtzBlobTriggerBeanServiceServer,
qrtz_calendars::qrtz_calendar_bean_service_server::QrtzCalendarBeanServiceServer,
qrtz_cron_triggers::qrtz_cron_triggers_bean_service_server::QrtzCronTriggersBeanServiceServer,
qrtz_fired_triggers::qrtz_fired_triggers_bean_service_server::QrtzFiredTriggersBeanServiceServer,
qrtz_job_details::qrtz_job_details_bean_service_server::QrtzJobDetailsBeanServiceServer,
qrtz_locks::qrtz_locks_bean_service_server::QrtzLocksBeanServiceServer,
qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_bean_service_server::QrtzPausedTriggerGrpsBeanServiceServer,
qrtz_scheduler_state::qrtz_scheduler_state_bean_service_server::QrtzSchedulerStateBeanServiceServer,
qrtz_simple_triggers::qrtz_simple_trigger_bean_service_server::QrtzSimpleTriggerBeanServiceServer,
qrtz_simprop_triggers::qrtz_simprop_trigger_bean_service_server::QrtzSimpropTriggerBeanServiceServer,
qrtz_triggers::qrtz_trigger_bean_service_server::QrtzTriggerBeanServiceServer,

};
use sea_orm::DatabaseConnection;


#[derive(Default, Debug, Clone)]
pub struct DolphinRpcServer {
    pub conn: DatabaseConnection,
}
macro_rules! create_service {
    ($fn_name:ident, $service_type:ident) => {
        pub fn $fn_name(self) -> $service_type<Self> {
            $service_type::new(self)
        }
    };
}

impl DolphinRpcServer {
    create_service!(ds_access_token, DsAccessTokenBeanServiceServer);

    create_service!(ds_alert, DsAlertBeanServiceServer);

    create_service!(
        ds_alert_plugin_instance,
        DsAlertPluginInstanceBeanServiceServer
    );

    create_service!(ds_alert_send_status, DsAlertSendStatusBeanServiceServer);

    create_service!(ds_alertgroup, DsAlertGroupBeanServiceServer);

    create_service!(ds_audit_log, DsAuditLogBeanServiceServer);

    create_service!(ds_command, DsCommandBeanServiceServer);

    create_service!(ds_database, DsDatasourceBeanServiceServer);

    create_service!(ds_dq_comparison_type, DsDqComparisonTypeBeanServiceServer);

    create_service!(ds_dq_execute_result, DsDqExecuteResultBeanServiceServer);

    create_service!(ds_dq_rule_service, DsDqRuleBeanServiceServer);

    create_service!(dq_rule_execute_sql, DsDqRuleExecuteSqlBeanServiceServer);

    create_service!(ds_dq_rule_input_entry, DsDqRuleInputEntryBeanServiceServer);

    create_service!(
        ds_dq_task_statistics_value,
        DsDqTaskStatisticsValueBeanServiceServer
    );

    create_service!(ds_environment, DsEnvironmentBeanServiceServer);

    create_service!(
        ds_environment_worker_group_relation,
        DsEnvironmentWorkerGroupRelationBeanServiceServer
    );

    create_service!(ds_error_command, DsErrorCommandBeanServiceServer);

    create_service!(ds_k8s, DsK8sBeanServiceServer);

    create_service!(ds_k8s_namespace, DsK8sNamespaceBeanServiceServer);

    create_service!(ds_plugin_define, DsPluginDefineBeanServiceServer);

    create_service!(ds_process_definition, DsProcessDefinitionBeanServiceServer);

    create_service!(
        ds_process_definition_log,
        DsProcessDefinitionLogBeanServiceServer
    );

    create_service!(ds_process_instance, DsProcessInstanceBeanServiceServer);

    create_service!(
        ds_process_task_relation,
        DsProcessTaskRelationBeanServiceServer
    );

    create_service!(
        ds_process_task_relation_log,
        DsProcessTaskRelationLogBeanServiceServer
    );

    create_service!(ds_project, DsProjectBeanServiceServer);

    create_service!(ds_queue, DsQueueBeanServiceServer);

    create_service!(
        ds_relation_datasource_user,
        DsRelationDatasourceUserBeanServiceServer
    );

    create_service!(
        ds_relation_namespace_user,
        DsRelationNamespaceUserBeanServiceServer
    );

    create_service!(
        ds_relation_process_instance,
        DsRelationProcessInstanceBeanServiceServer
    );

    create_service!(
        ds_relation_project_user,
        DsRelationProjectUserBeanServiceServer
    );

    create_service!(
        ds_relation_resources_user,
        DsRelationResourcesUserBeanServiceServer
    );

    create_service!(
        ds_relation_rule_execute_sql,
        DsRelationRuleExecuteSqlBeanServiceServer
    );

    create_service!(
        ds_relation_rule_input_entry,
        DsRelationRuleInputEntryBeanServiceServer
    );

    create_service!(ds_relation_udfs_user, DsRelationUdfsUserBeanServiceServer);

    create_service!(ds_resources, DsResourceBeanServiceServer);

    create_service!(ds_schedules, DsSchedulesBeanServiceServer);

    create_service!(ds_session, DsSessionBeanServiceServer);

    create_service!(ds_task_definition, DsTaskDefinitionBeanServiceServer);

    create_service!(ds_task_definition_log, DsTaskDefinitionLogBeanServiceServer);

    create_service!(ds_task_group, DsTaskGroupBeanServiceServer);

    create_service!(ds_task_group_queue, DsTaskGroupQueueBeanServiceServer);

    create_service!(ds_task_instance, DsTaskInstanceBeanServiceServer);

    create_service!(ds_tenant, DsTenantBeanServiceServer);

    create_service!(ds_udfs, DsUdfsBeanServiceServer);

    create_service!(ds_user, DsUserBeanServiceServer);

    create_service!(ds_version, DsVersionBeanServiceServer);

    create_service!(ds_worker_group, DsWorkerGroupBeanServiceServer);

    create_service!(qrtz_blob_triggers, QrtzBlobTriggerBeanServiceServer);

    create_service!(qrtz_calendars, QrtzCalendarBeanServiceServer);

    create_service!(qrtz_cron_triggers, QrtzCronTriggersBeanServiceServer);

    create_service!(qrtz_fired_triggers, QrtzFiredTriggersBeanServiceServer);

    create_service!(qrtz_job_details, QrtzJobDetailsBeanServiceServer);

    create_service!(qrtz_locks, QrtzLocksBeanServiceServer);

    create_service!(
        qrtz_paused_trigger_grps,
        QrtzPausedTriggerGrpsBeanServiceServer
    );

    create_service!(qrtz_scheduler_state, QrtzSchedulerStateBeanServiceServer);

    create_service!(qrtz_simple_triggers, QrtzSimpleTriggerBeanServiceServer);

    create_service!(qrtz_simprop_triggers, QrtzSimpropTriggerBeanServiceServer);

    create_service!(qrtz_triggers, QrtzTriggerBeanServiceServer);

    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}
