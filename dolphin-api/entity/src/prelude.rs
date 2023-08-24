//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

pub use super::{
    qrtz_blob_triggers::Entity as QrtzBlobTriggers,
    qrtz_calendars::Entity as QrtzCalendars,
    qrtz_cron_triggers::Entity as QrtzCronTriggers,
    qrtz_fired_triggers::Entity as QrtzFiredTriggers,
    qrtz_job_details::Entity as QrtzJobDetails,
    qrtz_locks::Entity as QrtzLocks,
    qrtz_paused_trigger_grps::Entity as QrtzPausedTriggerGrps,
    qrtz_scheduler_state::Entity as QrtzSchedulerState,
    qrtz_simple_triggers::Entity as QrtzSimpleTriggers,
    qrtz_simprop_triggers::Entity as QrtzSimpropTriggers,
    qrtz_triggers::Entity as QrtzTriggers,
    t_ds_access_token::Entity as TDsAccessToken,
    t_ds_alert::Entity as TDsAlert,
    t_ds_alert_plugin_instance::Entity as TDsAlertPluginInstance,
    t_ds_alert_send_status::Entity as TDsAlertSendStatus,
    t_ds_alertgroup::Entity as TDsAlertgroup,
    t_ds_audit_log::Entity as TDsAuditLog,
    t_ds_command::Entity as TDsCommand,
    t_ds_datasource::Entity as TDsDatasource,
    t_ds_dq_comparison_type::Entity as TDsDqComparisonType,
    t_ds_dq_execute_result::Entity as TDsDqExecuteResult,
    t_ds_dq_rule::Entity as TDsDqRule,
    t_ds_dq_rule_execute_sql::Entity as TDsDqRuleExecuteSql,
    t_ds_dq_rule_input_entry::Entity as TDsDqRuleInputEntry,
    t_ds_dq_task_statistics_value::Entity as TDsDqTaskStatisticsValue,
    t_ds_environment::Entity as TDsEnvironment,
    t_ds_environment_worker_group_relation::Entity as TDsEnvironmentWorkerGroupRelation,
    t_ds_error_command::Entity as TDsErrorCommand,
    t_ds_k8s::Entity as TDsK8s,
    t_ds_k8s_namespace::Entity as TDsK8sNamespace,
    t_ds_plugin_define::Entity as TDsPluginDefine,
    t_ds_process_definition::Entity as TDsProcessDefinition,
    t_ds_process_definition_log::Entity as TDsProcessDefinitionLog,
    t_ds_process_instance::Entity as TDsProcessInstance,
    t_ds_process_task_relation::Entity as TDsProcessTaskRelation,
    t_ds_process_task_relation_log::Entity as TDsProcessTaskRelationLog,
    t_ds_project::Entity as TDsProject,
    t_ds_queue::Entity as TDsQueue,
    t_ds_relation_datasource_user::Entity as TDsRelationDatasourceUser,
    t_ds_relation_namespace_user::Entity as TDsRelationNamespaceUser,
    t_ds_relation_process_instance::Entity as TDsRelationProcessInstance,
    t_ds_relation_project_user::Entity as TDsRelationProjectUser,
    t_ds_relation_resources_user::Entity as TDsRelationResourcesUser,
    t_ds_relation_rule_execute_sql::Entity as TDsRelationRuleExecuteSql,
    t_ds_relation_rule_input_entry::Entity as TDsRelationRuleInputEntry,
    t_ds_relation_udfs_user::Entity as TDsRelationUdfsUser,
    t_ds_resources::Entity as TDsResources,
    t_ds_schedules::Entity as TDsSchedules,
    t_ds_session::Entity as TDsSession,
    t_ds_task_definition::Entity as TDsTaskDefinition,
    t_ds_task_definition_log::Entity as TDsTaskDefinitionLog,
    t_ds_task_group::Entity as TDsTaskGroup,
    t_ds_task_group_queue::Entity as TDsTaskGroupQueue,
    t_ds_task_instance::Entity as TDsTaskInstance,
    t_ds_tenant::Entity as TDsTenant,
    t_ds_udfs::Entity as TDsUdfs,
    t_ds_user::Entity as TDsUser,
    t_ds_version::Entity as TDsVersion,
    t_ds_worker_group::Entity as TDsWorkerGroup,
};