use proto::{

ds_access_token::ds_access_token_bean_service_client::DsAccessTokenBeanServiceClient,
ds_alert::ds_alert_bean_service_client::DsAlertBeanServiceClient,
ds_alert_plugin_instance::ds_alert_plugin_instance_bean_service_client::DsAlertPluginInstanceBeanServiceClient,
ds_alert_send_status::ds_alert_send_status_bean_service_client::DsAlertSendStatusBeanServiceClient,
ds_alertgroup::ds_alert_group_bean_service_client::DsAlertGroupBeanServiceClient,
ds_audit_log::ds_audit_log_bean_service_client::DsAuditLogBeanServiceClient,
ds_command::ds_command_bean_service_client::DsCommandBeanServiceClient,
ds_datasource::ds_datasource_bean_service_client::DsDatasourceBeanServiceClient,
ds_dq_comparison_type::ds_dq_comparison_type_bean_service_client::DsDqComparisonTypeBeanServiceClient,
ds_dq_execute_result::ds_dq_execute_result_bean_service_client::DsDqExecuteResultBeanServiceClient,
ds_dq_rule::ds_dq_rule_bean_service_client::DsDqRuleBeanServiceClient,
ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_bean_service_client::DsDqRuleExecuteSqlBeanServiceClient,
ds_dq_rule_input_entry::ds_dq_rule_input_entry_bean_service_client::DsDqRuleInputEntryBeanServiceClient,
ds_dq_task_statistics_value::ds_dq_task_statistics_value_bean_service_client::DsDqTaskStatisticsValueBeanServiceClient,
ds_environment::ds_environment_bean_service_client::DsEnvironmentBeanServiceClient,
ds_environment_worker_group_relation::ds_environment_worker_group_relation_bean_service_client::DsEnvironmentWorkerGroupRelationBeanServiceClient,
ds_error_command::ds_error_command_bean_service_client::DsErrorCommandBeanServiceClient,
ds_k8s::ds_k8s_bean_service_client::DsK8sBeanServiceClient,
ds_k8s_namespace::ds_k8s_namespace_bean_service_client::DsK8sNamespaceBeanServiceClient,
ds_plugin_define::ds_plugin_define_bean_service_client::DsPluginDefineBeanServiceClient,
ds_process_definition::ds_process_definition_bean_service_client::DsProcessDefinitionBeanServiceClient,
ds_process_definition_log::ds_process_definition_log_bean_service_client::DsProcessDefinitionLogBeanServiceClient,
ds_process_instance::ds_process_instance_bean_service_client::DsProcessInstanceBeanServiceClient,
ds_process_task_relation::ds_process_task_relation_bean_service_client::DsProcessTaskRelationBeanServiceClient,
ds_process_task_relation_log::ds_process_task_relation_log_bean_service_client::DsProcessTaskRelationLogBeanServiceClient,
ds_project::ds_project_bean_service_client::DsProjectBeanServiceClient,
ds_queue::ds_queue_bean_service_client::DsQueueBeanServiceClient,
ds_relation_datasource_user::ds_relation_datasource_user_bean_service_client::DsRelationDatasourceUserBeanServiceClient,
ds_relation_namespace_user::ds_relation_namespace_user_bean_service_client::DsRelationNamespaceUserBeanServiceClient,
ds_relation_process_instance::ds_relation_process_instance_bean_service_client::DsRelationProcessInstanceBeanServiceClient,
ds_relation_project_user::ds_relation_project_user_bean_service_client::DsRelationProjectUserBeanServiceClient,
ds_relation_resources_user::ds_relation_resources_user_bean_service_client::DsRelationResourcesUserBeanServiceClient,
ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_bean_service_client::DsRelationRuleExecuteSqlBeanServiceClient,
ds_relation_rule_input_entry::ds_relation_rule_input_entry_bean_service_client::DsRelationRuleInputEntryBeanServiceClient,
ds_relation_udfs_user::ds_relation_udfs_user_bean_service_client::DsRelationUdfsUserBeanServiceClient,
ds_resources::ds_resource_bean_service_client::DsResourceBeanServiceClient,
ds_schedules::ds_schedules_bean_service_client::DsSchedulesBeanServiceClient,
ds_session::ds_session_bean_service_client::DsSessionBeanServiceClient,
ds_task_definition::ds_task_definition_bean_service_client::DsTaskDefinitionBeanServiceClient,
ds_task_definition_log::ds_task_definition_log_bean_service_client::DsTaskDefinitionLogBeanServiceClient,
ds_task_group::ds_task_group_bean_service_client::DsTaskGroupBeanServiceClient,
ds_task_group_queue::ds_task_group_queue_bean_service_client::DsTaskGroupQueueBeanServiceClient,
ds_task_instance::ds_task_instance_bean_service_client::DsTaskInstanceBeanServiceClient,
ds_tenant::ds_tenant_bean_service_client::DsTenantBeanServiceClient,
ds_udfs::ds_udfs_bean_service_client::DsUdfsBeanServiceClient,
ds_user::ds_user_bean_service_client::DsUserBeanServiceClient,
ds_version::ds_version_bean_service_client::DsVersionBeanServiceClient,
ds_worker_group::ds_worker_group_bean_service_client::DsWorkerGroupBeanServiceClient,
qrtz_blob_triggers::qrtz_blob_trigger_bean_service_client::QrtzBlobTriggerBeanServiceClient,
qrtz_calendars::qrtz_calendar_bean_service_client::QrtzCalendarBeanServiceClient,
qrtz_cron_triggers::qrtz_cron_triggers_bean_service_client::QrtzCronTriggersBeanServiceClient,
qrtz_fired_triggers::qrtz_fired_triggers_bean_service_client::QrtzFiredTriggersBeanServiceClient,
qrtz_job_details::qrtz_job_details_bean_service_client::QrtzJobDetailsBeanServiceClient,
qrtz_locks::qrtz_locks_bean_service_client::QrtzLocksBeanServiceClient,
qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_bean_service_client::QrtzPausedTriggerGrpsBeanServiceClient,
qrtz_scheduler_state::qrtz_scheduler_state_bean_service_client::QrtzSchedulerStateBeanServiceClient,
qrtz_simple_triggers::qrtz_simple_trigger_bean_service_client::QrtzSimpleTriggerBeanServiceClient,
qrtz_simprop_triggers::qrtz_simprop_trigger_bean_service_client::QrtzSimpropTriggerBeanServiceClient,
qrtz_triggers::qrtz_trigger_bean_service_client::QrtzTriggerBeanServiceClient,
};
macro_rules! get_client {
    ($fn_name:ident, $service_type:ident) => {
        pub async fn $fn_name() -> anyhow::Result<$service_type<tonic::transport::Channel>> {
            dotenvy::dotenv().ok();
            let host = std::env::var("DOLPHIN_DAO_HOST")
                .expect("HOST is not set in .env file")
                .clone();
            let port = std::env::var("DOLPHIN_DAO_PORT")
                .expect("PORT is not set in .env file")
                .clone();
            let addr_str = format!("http://{host}:{port}").clone();
            let addr = tonic::transport::Endpoint::from_shared(addr_str);
            match $service_type::connect(addr.unwrap()).await {
                Ok(client) => Ok(client),
                Err(e) => Err(anyhow::Error::new(e)),
            }
        }
    };
}

macro_rules! init_const_client {
    ($contanst_name:ident, $service_type:ident) => {
        pub static $contanst_name: tokio::sync::OnceCell<
            anyhow::Result<$service_type<tonic::transport::Channel>>,
        > = tokio::sync::OnceCell::const_new();
    };
}

init_const_client!(ACCESS_TOKEN_SERVICE, DsAccessTokenBeanServiceClient);
init_const_client!(ALERT_SERVICE, DsAlertBeanServiceClient);
init_const_client!(
    ALERT_PLUGIN_INSTANCE_SERVICE,
    DsAlertPluginInstanceBeanServiceClient
);
init_const_client!(
    ALERT_SEND_STATUS_SERVICE,
    DsAlertSendStatusBeanServiceClient
);
init_const_client!(ALERT_GROUP_SERVICE, DsAlertGroupBeanServiceClient);
init_const_client!(AUDIT_LOG_SERVICE, DsAuditLogBeanServiceClient);
init_const_client!(COMMAND_SERVICE, DsCommandBeanServiceClient);
init_const_client!(DATASOURCE_SERVICE, DsDatasourceBeanServiceClient);
init_const_client!(
    DQ_COMPARISON_TYPE_SERVICE,
    DsDqComparisonTypeBeanServiceClient
);
init_const_client!(
    DQ_EXECUTE_RESULT_SERVICE,
    DsDqExecuteResultBeanServiceClient
);
init_const_client!(DQ_RULE_SERVICE, DsDqRuleBeanServiceClient);
init_const_client!(
    DQ_RULE_EXECUTE_SQL_SERVICE,
    DsDqRuleExecuteSqlBeanServiceClient
);
init_const_client!(
    DQ_RULE_INPUT_ENTRY_SERVICE,
    DsDqRuleInputEntryBeanServiceClient
);
init_const_client!(
    DQ_TASK_STATISTICS_VALUE_SERVICE,
    DsDqTaskStatisticsValueBeanServiceClient
);
init_const_client!(ENVIRONMENT_SERVICE, DsEnvironmentBeanServiceClient);
init_const_client!(
    ENVIRONMENT_WORKER_GROUP_RELATION_SERVICE,
    DsEnvironmentWorkerGroupRelationBeanServiceClient
);
init_const_client!(ERROR_COMMAND_SERVICE, DsErrorCommandBeanServiceClient);
init_const_client!(K8S_SERVICE, DsK8sBeanServiceClient);
init_const_client!(K8S_NAMESPACE_SERVICE, DsK8sNamespaceBeanServiceClient);
init_const_client!(PLUGIN_DEFINE_SERVICE, DsPluginDefineBeanServiceClient);
init_const_client!(
    PROCESS_DEFINITION_SERVICE,
    DsProcessDefinitionBeanServiceClient
);
init_const_client!(
    PROCESS_DEFINITION_LOG_SERVICE,
    DsProcessDefinitionLogBeanServiceClient
);
init_const_client!(PROCESS_INSTANCE_SERVICE, DsProcessInstanceBeanServiceClient);
init_const_client!(
    PROCESS_TASK_RELATION_SERVICE,
    DsProcessTaskRelationBeanServiceClient
);
init_const_client!(
    PROCESS_TASK_RELATION_LOG_SERVICE,
    DsProcessTaskRelationLogBeanServiceClient
);
init_const_client!(PROJECT_SERVICE, DsProjectBeanServiceClient);
init_const_client!(QUEUE_SERVICE, DsQueueBeanServiceClient);
init_const_client!(
    RELATION_DATASOURCE_USER_SERVICE,
    DsRelationDatasourceUserBeanServiceClient
);
init_const_client!(
    RELATION_NAMESPACE_USER_SERVICE,
    DsRelationNamespaceUserBeanServiceClient
);
init_const_client!(
    RELATION_PROCESS_INSTANCE_SERVICE,
    DsRelationProcessInstanceBeanServiceClient
);
init_const_client!(
    RELATION_PROJECT_USER_SERVICE,
    DsRelationProjectUserBeanServiceClient
);
init_const_client!(
    RELATION_RESOURCES_USER_SERVICE,
    DsRelationResourcesUserBeanServiceClient
);
init_const_client!(
    RELATION_RULE_EXECUTE_SQL_SERVICE,
    DsRelationRuleExecuteSqlBeanServiceClient
);
init_const_client!(
    RELATION_RULE_INPUT_ENTRY_SERVICE,
    DsRelationRuleInputEntryBeanServiceClient
);
init_const_client!(
    RELATION_UDFS_USER_SERVICE,
    DsRelationUdfsUserBeanServiceClient
);
init_const_client!(RESOURCE_SERVICE, DsResourceBeanServiceClient);
init_const_client!(SCHEDULES_SERVICE, DsSchedulesBeanServiceClient);
init_const_client!(SESSION_SERVICE, DsSessionBeanServiceClient);
init_const_client!(TASK_DEFINITION_SERVICE, DsTaskDefinitionBeanServiceClient);
init_const_client!(
    TASK_DEFINITION_LOG_SERVICE,
    DsTaskDefinitionLogBeanServiceClient
);
init_const_client!(TASK_GROUP_SERVICE, DsTaskGroupBeanServiceClient);
init_const_client!(TASK_GROUP_QUEUE_SERVICE, DsTaskGroupQueueBeanServiceClient);
init_const_client!(TASK_INSTANCE_SERVICE, DsTaskInstanceBeanServiceClient);
init_const_client!(TENANT_SERVICE, DsTenantBeanServiceClient);
init_const_client!(UDFS_SERVICE, DsUdfsBeanServiceClient);
init_const_client!(USER_SERVICE, DsUserBeanServiceClient);
init_const_client!(VERSION_SERVICE, DsVersionBeanServiceClient);
init_const_client!(WORKER_GROUP_SERVICE, DsWorkerGroupBeanServiceClient);
init_const_client!(QRTZ_BLOB_TRIGGER_SERVICE, QrtzBlobTriggerBeanServiceClient);
init_const_client!(QRTZ_CALENDAR_SERVICE, QrtzCalendarBeanServiceClient);
init_const_client!(
    QRTZ_CRON_TRIGGERS_SERVICE,
    QrtzCronTriggersBeanServiceClient
);
init_const_client!(
    QRTZ_FIRED_TRIGGERS_SERVICE,
    QrtzFiredTriggersBeanServiceClient
);
init_const_client!(QRTZ_JOB_DETAILS_SERVICE, QrtzJobDetailsBeanServiceClient);
init_const_client!(QRTZ_LOCKS_SERVICE, QrtzLocksBeanServiceClient);
init_const_client!(
    QRTZ_PAUSED_TRIGGER_GRPS_SERVICE,
    QrtzPausedTriggerGrpsBeanServiceClient
);
init_const_client!(
    QRTZ_SCHEDULER_STATE_SERVICE,
    QrtzSchedulerStateBeanServiceClient
);
init_const_client!(
    QRTZ_SIMPLE_TRIGGER_SERVICE,
    QrtzSimpleTriggerBeanServiceClient
);
init_const_client!(
    QRTZ_SIMPROP_TRIGGER_SERVICE,
    QrtzSimpropTriggerBeanServiceClient
);
init_const_client!(QRTZ_TRIGGER_SERVICE, QrtzTriggerBeanServiceClient);
get_client!(access_token_client, DsAccessTokenBeanServiceClient);
get_client!(alert_client, DsAlertBeanServiceClient);
get_client!(
    alert_plugin_instance_client,
    DsAlertPluginInstanceBeanServiceClient
);
get_client!(alert_send_status_client, DsAlertSendStatusBeanServiceClient);
get_client!(alert_group_client, DsAlertGroupBeanServiceClient);
get_client!(audit_log_client, DsAuditLogBeanServiceClient);
get_client!(command_client, DsCommandBeanServiceClient);
get_client!(datasource_client, DsDatasourceBeanServiceClient);
get_client!(
    dq_comparison_type_client,
    DsDqComparisonTypeBeanServiceClient
);
get_client!(dq_execute_result_client, DsDqExecuteResultBeanServiceClient);
get_client!(dq_rule_client, DsDqRuleBeanServiceClient);
get_client!(
    dq_rule_execute_sql_client,
    DsDqRuleExecuteSqlBeanServiceClient
);
get_client!(
    dq_rule_input_entry_client,
    DsDqRuleInputEntryBeanServiceClient
);
get_client!(
    dq_task_statistics_value_client,
    DsDqTaskStatisticsValueBeanServiceClient
);
get_client!(environment_client, DsEnvironmentBeanServiceClient);
get_client!(
    environment_worker_group_relation_client,
    DsEnvironmentWorkerGroupRelationBeanServiceClient
);
get_client!(error_command_client, DsErrorCommandBeanServiceClient);
get_client!(k8s_client, DsK8sBeanServiceClient);
get_client!(k8s_namespace_client, DsK8sNamespaceBeanServiceClient);
get_client!(plugin_define_client, DsPluginDefineBeanServiceClient);
get_client!(
    process_definition_client,
    DsProcessDefinitionBeanServiceClient
);
get_client!(
    process_definition_log_client,
    DsProcessDefinitionLogBeanServiceClient
);
get_client!(process_instance_client, DsProcessInstanceBeanServiceClient);
get_client!(
    process_task_relation_client,
    DsProcessTaskRelationBeanServiceClient
);
get_client!(
    process_task_relation_log_client,
    DsProcessTaskRelationLogBeanServiceClient
);
get_client!(project_client, DsProjectBeanServiceClient);
get_client!(queue_client, DsQueueBeanServiceClient);
get_client!(
    relation_datasource_user_client,
    DsRelationDatasourceUserBeanServiceClient
);
get_client!(
    relation_namespace_user_client,
    DsRelationNamespaceUserBeanServiceClient
);
get_client!(
    relation_process_instance_client,
    DsRelationProcessInstanceBeanServiceClient
);
get_client!(
    relation_project_user_client,
    DsRelationProjectUserBeanServiceClient
);
get_client!(
    relation_resources_user_client,
    DsRelationResourcesUserBeanServiceClient
);
get_client!(
    relation_rule_execute_sql_client,
    DsRelationRuleExecuteSqlBeanServiceClient
);
get_client!(
    relation_rule_input_entry_client,
    DsRelationRuleInputEntryBeanServiceClient
);
get_client!(
    relation_udfs_user_client,
    DsRelationUdfsUserBeanServiceClient
);
get_client!(resource_client, DsResourceBeanServiceClient);
get_client!(schedules_client, DsSchedulesBeanServiceClient);
get_client!(session_client, DsSessionBeanServiceClient);
get_client!(task_definition_client, DsTaskDefinitionBeanServiceClient);
get_client!(
    task_definition_log_client,
    DsTaskDefinitionLogBeanServiceClient
);
get_client!(task_group_client, DsTaskGroupBeanServiceClient);
get_client!(task_group_queue_client, DsTaskGroupQueueBeanServiceClient);
get_client!(task_instance_client, DsTaskInstanceBeanServiceClient);
get_client!(tenant_client, DsTenantBeanServiceClient);
get_client!(udfs_client, DsUdfsBeanServiceClient);
get_client!(user_client, DsUserBeanServiceClient);
get_client!(version_client, DsVersionBeanServiceClient);
get_client!(worker_group_client, DsWorkerGroupBeanServiceClient);
get_client!(qrtz_blob_trigger_client, QrtzBlobTriggerBeanServiceClient);
get_client!(qrtz_calendar_client, QrtzCalendarBeanServiceClient);
get_client!(qrtz_cron_triggers_client, QrtzCronTriggersBeanServiceClient);
get_client!(
    qrtz_fired_triggers_client,
    QrtzFiredTriggersBeanServiceClient
);
get_client!(qrtz_job_details_client, QrtzJobDetailsBeanServiceClient);
get_client!(qrtz_locks_client, QrtzLocksBeanServiceClient);
get_client!(
    qrtz_paused_trigger_grps_client,
    QrtzPausedTriggerGrpsBeanServiceClient
);
get_client!(
    qrtz_scheduler_state_client,
    QrtzSchedulerStateBeanServiceClient
);
get_client!(
    qrtz_simple_trigger_client,
    QrtzSimpleTriggerBeanServiceClient
);
get_client!(
    qrtz_simprop_trigger_client,
    QrtzSimpropTriggerBeanServiceClient
);
get_client!(qrtz_trigger_client, QrtzTriggerBeanServiceClient);


// pub static USER_SERVICE: OnceCell<Result<DsUserBeanServiceClient<Channel>>> =
//     OnceCell::const_new();


// pub async fn get_client() -> Result<DsUserBeanServiceClient<Channel>> {
//     dotenvy::dotenv().ok();
//     let host = env::var("DOLPHIN_DAO_HOST")
//         .expect("HOST is not set in .env file")
//         .clone();
//     let port = env::var("DOLPHIN_DAO_PORT")
//         .expect("PORT is not set in .env file")
//         .clone();
//     let addr_str = format!("http://{host}:{port}").clone();
//     let addr = Endpoint::from_shared(addr_str);
//     match DsUserBeanServiceClient::connect(addr.unwrap()).await {
//         Ok(client) => Ok(client),
//         Err(e) => Err(anyhow::Error::new(e)),
//     }
// }
