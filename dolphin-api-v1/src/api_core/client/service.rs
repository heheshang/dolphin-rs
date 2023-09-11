use dolphin_config::dao_config::Settings;
use proto::{
    ds_access_token::ds_access_token_service_client::DsAccessTokenServiceClient,
    ds_alert::ds_alert_service_client::DsAlertServiceClient,
    ds_alert_plugin_instance::ds_alert_plugin_instance_service_client::DsAlertPluginInstanceServiceClient,
    ds_alert_send_status::ds_alert_send_status_service_client::DsAlertSendStatusServiceClient,
    ds_alertgroup::ds_alert_group_service_client::DsAlertGroupServiceClient,
    ds_audit_log::ds_audit_log_service_client::DsAuditLogServiceClient,
    ds_command::ds_command_service_client::DsCommandServiceClient,
    ds_datasource::ds_datasource_service_client::DsDatasourceServiceClient,
    ds_dq_comparison_type::ds_dq_comparison_type_service_client::DsDqComparisonTypeServiceClient,
    ds_dq_execute_result::ds_dq_execute_result_service_client::DsDqExecuteResultServiceClient,
    ds_dq_rule::ds_dq_rule_service_client::DsDqRuleServiceClient,
    ds_dq_rule_execute_sql::ds_dq_rule_execute_sql_service_client::DsDqRuleExecuteSqlServiceClient,
    ds_dq_rule_input_entry::ds_dq_rule_input_entry_service_client::DsDqRuleInputEntryServiceClient,
    ds_dq_task_statistics_value::ds_dq_task_statistics_value_service_client::DsDqTaskStatisticsValueServiceClient,
    ds_environment::ds_environment_service_client::DsEnvironmentServiceClient,
    ds_environment_worker_group_relation::ds_environment_worker_group_relation_service_client::DsEnvironmentWorkerGroupRelationServiceClient,
    ds_error_command::ds_error_command_service_client::DsErrorCommandServiceClient,
    ds_k8s::ds_k8s_service_client::DsK8sServiceClient,
    ds_k8s_namespace::ds_k8s_namespace_service_client::DsK8sNamespaceServiceClient,
    ds_plugin_define::ds_plugin_define_service_client::DsPluginDefineServiceClient,
    ds_process_definition::ds_process_definition_service_client::DsProcessDefinitionServiceClient,
    ds_process_definition_log::ds_process_definition_log_service_client::DsProcessDefinitionLogServiceClient,
    ds_process_instance::ds_process_instance_service_client::DsProcessInstanceServiceClient,
    ds_process_task_relation::ds_process_task_relation_service_client::DsProcessTaskRelationServiceClient,
    ds_process_task_relation_log::ds_process_task_relation_log_service_client::DsProcessTaskRelationLogServiceClient,
    ds_project::ds_project_service_client::DsProjectServiceClient,
    ds_queue::ds_queue_service_client::DsQueueServiceClient,
    ds_relation_datasource_user::ds_relation_datasource_user_service_client::DsRelationDatasourceUserServiceClient,
    ds_relation_namespace_user::ds_relation_namespace_user_service_client::DsRelationNamespaceUserServiceClient,
    ds_relation_process_instance::ds_relation_process_instance_service_client::DsRelationProcessInstanceServiceClient,
    ds_relation_project_user::ds_relation_project_user_service_client::DsRelationProjectUserServiceClient,
    ds_relation_resources_user::ds_relation_resources_user_service_client::DsRelationResourcesUserServiceClient,
    ds_relation_rule_execute_sql::ds_relation_rule_execute_sql_service_client::DsRelationRuleExecuteSqlServiceClient,
    ds_relation_rule_input::ds_relation_rule_input_service_client::DsRelationRuleInputServiceClient,
    ds_relation_udfs_user::ds_relation_udfs_user_service_client::DsRelationUdfsUserServiceClient,
    ds_resources::ds_resource_service_client::DsResourceServiceClient,
    ds_schedules::ds_schedules_service_client::DsSchedulesServiceClient,
    ds_session::ds_session_service_client::DsSessionServiceClient,
    ds_task_definition::ds_task_definition_service_client::DsTaskDefinitionServiceClient,
    ds_task_definition_log::ds_task_definition_log_service_client::DsTaskDefinitionLogServiceClient,
    ds_task_group::ds_task_group_service_client::DsTaskGroupServiceClient,
    ds_task_group_queue::ds_task_group_queue_service_client::DsTaskGroupQueueServiceClient,
    ds_task_instance::ds_task_instance_service_client::DsTaskInstanceServiceClient,
    ds_tenant::ds_tenant_service_client::DsTenantServiceClient,
    ds_udfs::ds_udfs_service_client::DsUdfsServiceClient,
    ds_user::ds_user_service_client::DsUserServiceClient,
    ds_version::ds_version_service_client::DsVersionServiceClient,
    ds_worker_group::ds_worker_group_service_client::DsWorkerGroupServiceClient,
    qrtz_blob_triggers::qrtz_blob_trigger_service_client::QrtzBlobTriggerServiceClient,
    qrtz_calendars::qrtz_calendar_service_client::QrtzCalendarServiceClient,
    qrtz_cron_triggers::qrtz_cron_triggers_service_client::QrtzCronTriggersServiceClient,
    qrtz_fired_triggers::qrtz_fired_triggers_service_client::QrtzFiredTriggersServiceClient,
    qrtz_job_details::qrtz_job_details_service_client::QrtzJobDetailsServiceClient,
    qrtz_locks::qrtz_locks_service_client::QrtzLocksServiceClient,
    qrtz_paused_trigger_grps::qrtz_paused_trigger_grps_service_client::QrtzPausedTriggerGrpsServiceClient,
    qrtz_scheduler_state::qrtz_scheduler_state_service_client::QrtzSchedulerStateServiceClient,
    qrtz_simple_triggers::qrtz_simple_trigger_service_client::QrtzSimpleTriggerServiceClient,
    qrtz_simprop_triggers::qrtz_simprop_trigger_service_client::QrtzSimpropTriggerServiceClient,
    qrtz_triggers::qrtz_trigger_service_client::QrtzTriggerServiceClient,
};
macro_rules! get_client {
    ($fn_name:ident, $service_type:ident) => {
        pub async fn $fn_name() -> anyhow::Result<$service_type<tonic::transport::Channel>> {
            let settings = Settings::new().unwrap();
            let host = settings.server.host;
            let port = settings.server.port;

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

init_const_client!(_ACCESS_TOKEN_SERVICE, DsAccessTokenServiceClient);
init_const_client!(_ALERT_SERVICE, DsAlertServiceClient);
init_const_client!(
    _ALERT_PLUGIN_INSTANCE_SERVICE,
    DsAlertPluginInstanceServiceClient
);
init_const_client!(_ALERT_SEND_STATUS_SERVICE, DsAlertSendStatusServiceClient);
init_const_client!(_ALERT_GROUP_SERVICE, DsAlertGroupServiceClient);
init_const_client!(_AUDIT_LOG_SERVICE, DsAuditLogServiceClient);
init_const_client!(_COMMAND_SERVICE, DsCommandServiceClient);
init_const_client!(_DATASOURCE_SERVICE, DsDatasourceServiceClient);
init_const_client!(_DQ_COMPARISON_TYPE_SERVICE, DsDqComparisonTypeServiceClient);
init_const_client!(_DQ_EXECUTE_RESULT_SERVICE, DsDqExecuteResultServiceClient);
init_const_client!(_DQ_RULE_SERVICE, DsDqRuleServiceClient);
init_const_client!(
    _DQ_RULE_EXECUTE_SQL_SERVICE,
    DsDqRuleExecuteSqlServiceClient
);
init_const_client!(
    _DQ_RULE_INPUT_ENTRY_SERVICE,
    DsDqRuleInputEntryServiceClient
);
init_const_client!(
    _DQ_TASK_STATISTICS_VALUE_SERVICE,
    DsDqTaskStatisticsValueServiceClient
);
init_const_client!(_ENVIRONMENT_SERVICE, DsEnvironmentServiceClient);
init_const_client!(
    _ENVIRONMENT_WORKER_GROUP_RELATION_SERVICE,
    DsEnvironmentWorkerGroupRelationServiceClient
);
init_const_client!(_ERROR_COMMAND_SERVICE, DsErrorCommandServiceClient);
init_const_client!(_K8S_SERVICE, DsK8sServiceClient);
init_const_client!(_K8S_NAMESPACE_SERVICE, DsK8sNamespaceServiceClient);
init_const_client!(_PLUGIN_DEFINE_SERVICE, DsPluginDefineServiceClient);
init_const_client!(
    _PROCESS_DEFINITION_SERVICE,
    DsProcessDefinitionServiceClient
);
init_const_client!(
    _PROCESS_DEFINITION_LOG_SERVICE,
    DsProcessDefinitionLogServiceClient
);
init_const_client!(_PROCESS_INSTANCE_SERVICE, DsProcessInstanceServiceClient);
init_const_client!(
    _PROCESS_TASK_RELATION_SERVICE,
    DsProcessTaskRelationServiceClient
);
init_const_client!(
    _PROCESS_TASK_RELATION_LOG_SERVICE,
    DsProcessTaskRelationLogServiceClient
);
init_const_client!(_PROJECT_SERVICE, DsProjectServiceClient);
init_const_client!(_QUEUE_SERVICE, DsQueueServiceClient);
init_const_client!(
    _RELATION_DATASOURCE_USER_SERVICE,
    DsRelationDatasourceUserServiceClient
);
init_const_client!(
    _RELATION_NAMESPACE_USER_SERVICE,
    DsRelationNamespaceUserServiceClient
);
init_const_client!(
    _RELATION_PROCESS_INSTANCE_SERVICE,
    DsRelationProcessInstanceServiceClient
);
init_const_client!(
    _RELATION_PROJECT_USER_SERVICE,
    DsRelationProjectUserServiceClient
);
init_const_client!(
    _RELATION_RESOURCES_USER_SERVICE,
    DsRelationResourcesUserServiceClient
);
init_const_client!(
    _RELATION_RULE_EXECUTE_SQL_SERVICE,
    DsRelationRuleExecuteSqlServiceClient
);
init_const_client!(
    _RELATION_RULE_INPUT_ENTRY_SERVICE,
    DsRelationRuleInputServiceClient
);
init_const_client!(_RELATION_UDFS_USER_SERVICE, DsRelationUdfsUserServiceClient);
init_const_client!(_RESOURCE_SERVICE, DsResourceServiceClient);
init_const_client!(_SCHEDULES_SERVICE, DsSchedulesServiceClient);
init_const_client!(SESSION_SERVICE, DsSessionServiceClient);
init_const_client!(_TASK_DEFINITION_SERVICE, DsTaskDefinitionServiceClient);
init_const_client!(
    _TASK_DEFINITION_LOG_SERVICE,
    DsTaskDefinitionLogServiceClient
);
init_const_client!(_TASK_GROUP_SERVICE, DsTaskGroupServiceClient);
init_const_client!(_TASK_GROUP_QUEUE_SERVICE, DsTaskGroupQueueServiceClient);
init_const_client!(_TASK_INSTANCE_SERVICE, DsTaskInstanceServiceClient);
init_const_client!(_TENANT_SERVICE, DsTenantServiceClient);
init_const_client!(_UDFS_SERVICE, DsUdfsServiceClient);
init_const_client!(USER_SERVICE, DsUserServiceClient);
init_const_client!(_VERSION_SERVICE, DsVersionServiceClient);
init_const_client!(_WORKER_GROUP_SERVICE, DsWorkerGroupServiceClient);
init_const_client!(_QRTZ_BLOB_TRIGGER_SERVICE, QrtzBlobTriggerServiceClient);
init_const_client!(_QRTZ_CALENDAR_SERVICE, QrtzCalendarServiceClient);
init_const_client!(_QRTZ_CRON_TRIGGERS_SERVICE, QrtzCronTriggersServiceClient);
init_const_client!(_QRTZ_FIRED_TRIGGERS_SERVICE, QrtzFiredTriggersServiceClient);
init_const_client!(_QRTZ_JOB_DETAILS_SERVICE, QrtzJobDetailsServiceClient);
init_const_client!(_QRTZ_LOCKS_SERVICE, QrtzLocksServiceClient);
init_const_client!(
    _QRTZ_PAUSED_TRIGGER_GRPS_SERVICE,
    QrtzPausedTriggerGrpsServiceClient
);
init_const_client!(
    _QRTZ_SCHEDULER_STATE_SERVICE,
    QrtzSchedulerStateServiceClient
);
init_const_client!(_QRTZ_SIMPLE_TRIGGER_SERVICE, QrtzSimpleTriggerServiceClient);
init_const_client!(
    _QRTZ_SIMPROP_TRIGGER_SERVICE,
    QrtzSimpropTriggerServiceClient
);
init_const_client!(_QRTZ_TRIGGER_SERVICE, QrtzTriggerServiceClient);
get_client!(_access_token_client, DsAccessTokenServiceClient);
get_client!(_alert_client, DsAlertServiceClient);
get_client!(
    _alert_plugin_instance_client,
    DsAlertPluginInstanceServiceClient
);
get_client!(_alert_send_status_client, DsAlertSendStatusServiceClient);
get_client!(_alert_group_client, DsAlertGroupServiceClient);
get_client!(_audit_log_client, DsAuditLogServiceClient);
get_client!(_command_client, DsCommandServiceClient);
get_client!(_datasource_client, DsDatasourceServiceClient);
get_client!(_dq_comparison_type_client, DsDqComparisonTypeServiceClient);
get_client!(_dq_execute_result_client, DsDqExecuteResultServiceClient);
get_client!(_dq_rule_client, DsDqRuleServiceClient);
get_client!(_dq_rule_execute_sql_client, DsDqRuleExecuteSqlServiceClient);
get_client!(_dq_rule_input_entry_client, DsDqRuleInputEntryServiceClient);
get_client!(
    _dq_task_statistics_value_client,
    DsDqTaskStatisticsValueServiceClient
);
get_client!(_environment_client, DsEnvironmentServiceClient);
get_client!(
    _environment_worker_group_relation_client,
    DsEnvironmentWorkerGroupRelationServiceClient
);
get_client!(_error_command_client, DsErrorCommandServiceClient);
get_client!(_k8s_client, DsK8sServiceClient);
get_client!(_k8s_namespace_client, DsK8sNamespaceServiceClient);
get_client!(_plugin_define_client, DsPluginDefineServiceClient);
get_client!(_process_definition_client, DsProcessDefinitionServiceClient);
get_client!(
    _process_definition_log_client,
    DsProcessDefinitionLogServiceClient
);
get_client!(_process_instance_client, DsProcessInstanceServiceClient);
get_client!(
    _process_task_relation_client,
    DsProcessTaskRelationServiceClient
);
get_client!(
    _process_task_relation_log_client,
    DsProcessTaskRelationLogServiceClient
);
get_client!(_project_client, DsProjectServiceClient);
get_client!(_queue_client, DsQueueServiceClient);
get_client!(
    _relation_datasource_user_client,
    DsRelationDatasourceUserServiceClient
);
get_client!(
    _relation_namespace_user_client,
    DsRelationNamespaceUserServiceClient
);
get_client!(
    _relation_process_instance_client,
    DsRelationProcessInstanceServiceClient
);
get_client!(
    _relation_project_user_client,
    DsRelationProjectUserServiceClient
);
get_client!(
    _relation_resources_user_client,
    DsRelationResourcesUserServiceClient
);
get_client!(
    _relation_rule_execute_sql_client,
    DsRelationRuleExecuteSqlServiceClient
);
get_client!(
    _relation_rule_input_entry_client,
    DsRelationRuleInputServiceClient
);
get_client!(_relation_udfs_user_client, DsRelationUdfsUserServiceClient);
get_client!(_resource_client, DsResourceServiceClient);
get_client!(_schedules_client, DsSchedulesServiceClient);
get_client!(_session_client, DsSessionServiceClient);
get_client!(_task_definition_client, DsTaskDefinitionServiceClient);
get_client!(
    _task_definition_log_client,
    DsTaskDefinitionLogServiceClient
);
get_client!(_task_group_client, DsTaskGroupServiceClient);
get_client!(_task_group_queue_client, DsTaskGroupQueueServiceClient);
get_client!(_task_instance_client, DsTaskInstanceServiceClient);
get_client!(_tenant_client, DsTenantServiceClient);
get_client!(_udfs_client, DsUdfsServiceClient);
get_client!(_user_client, DsUserServiceClient);
get_client!(_version_client, DsVersionServiceClient);
get_client!(_worker_group_client, DsWorkerGroupServiceClient);
get_client!(_qrtz_blob_trigger_client, QrtzBlobTriggerServiceClient);
get_client!(_qrtz_calendar_client, QrtzCalendarServiceClient);
get_client!(_qrtz_cron_triggers_client, QrtzCronTriggersServiceClient);
get_client!(_qrtz_fired_triggers_client, QrtzFiredTriggersServiceClient);
get_client!(_qrtz_job_details_client, QrtzJobDetailsServiceClient);
get_client!(_qrtz_locks_client, QrtzLocksServiceClient);
get_client!(
    _qrtz_paused_trigger_grps_client,
    QrtzPausedTriggerGrpsServiceClient
);
get_client!(
    _qrtz_scheduler_state_client,
    QrtzSchedulerStateServiceClient
);
get_client!(_qrtz_simple_trigger_client, QrtzSimpleTriggerServiceClient);
get_client!(
    _qrtz_simprop_trigger_client,
    QrtzSimpropTriggerServiceClient
);
get_client!(_qrtz_trigger_client, QrtzTriggerServiceClient);

// pub static USER_SERVICE: OnceCell<Result<DsUserServiceClient<Channel>>> =
//     OnceCell::const_new();

// pub async fn get_client() -> Result<DsUserServiceClient<Channel>> {
//     dotenvy::dotenv().ok();
//     let host = env::var("DOLPHIN_DAO_HOST")
//         .expect("HOST is not set in .env file")
//         .clone();
//     let port = env::var("DOLPHIN_DAO_PORT")
//         .expect("PORT is not set in .env file")
//         .clone();
//     let addr_str = format!("http://{host}:{port}").clone();
//     let addr = Endpoint::from_shared(addr_str);
//     match DsUserServiceClient::connect(addr.unwrap()).await {
//         Ok(client) => Ok(client),
//         Err(e) => Err(anyhow::Error::new(e)),
//     }
// }
