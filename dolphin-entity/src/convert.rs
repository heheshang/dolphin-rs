use crate::{
    qrtz_blob_triggers::Model as QrtzBlobTriggerModel,
    qrtz_calendars::Model as QrtzCalendarModel,
    qrtz_cron_triggers::Model as QrtzCronTriggersModel,
    qrtz_fired_triggers::Model as QrtzFiredTriggersModel,
    qrtz_job_details::Model as QrtzJobDetailsModel,
    qrtz_locks::Model as QrtzLocksModel,
    qrtz_paused_trigger_grps::Model as QrtzPausedTriggerGrpsModel,
    qrtz_scheduler_state::Model as QrtzSchedulerStateModel,
    qrtz_simple_triggers::Model as QrtzSimpleTriggersModel,
    qrtz_simprop_triggers::Model as QrtzSimpropTriggerModel,
    qrtz_triggers::Model as QrtzTriggersModel,
    t_ds_access_token::Model as DsAccessTokenModel,
    t_ds_alert::Model as DsAlertModel,
    t_ds_alert_plugin_instance::Model as DsAlertPluginInstanceModel,
    t_ds_alert_send_status::Model as DsAlertSendStatusModel,
    t_ds_alertgroup::Model as DsAlertgroupModel,
    t_ds_audit_log::Model as DsAuditLogModel,
    t_ds_command::Model as DsCommandModel,
    t_ds_datasource::Model as DsDatasourceModel,
    t_ds_dq_comparison_type::Model as DsDqComparisonTypeModel,
    t_ds_dq_execute_result::Model as DsDqExecuteResultModel,
    t_ds_dq_rule::Model as DsDqRuleModel,
    t_ds_dq_rule_execute_sql::Model as DsDqRuleExecuteSqlModel,
    t_ds_dq_rule_input_entry::Model as DsDqRuleInputEntryModel,
    t_ds_dq_task_statistics_value::Model as DsDqTaskStatisticsValueModel,
    t_ds_environment::Model as DsEnvironmentModel,
    t_ds_environment_worker_group_relation::Model as DsEnvironmentWorkerGroupRelationModel,
    t_ds_error_command::Model as DsErrorCommandModel,
    t_ds_k8s::Model as DsK8sModel,
    t_ds_k8s_namespace::Model as DsK8sNamespaceModel,
    t_ds_plugin_define::Model as DsPluginDefineModel,
    t_ds_process_definition::Model as DsProcessDefinitionModel,
    t_ds_process_definition_log::Model as DsProcessDefinitionLogModel,
    t_ds_process_instance::Model as DsProcessInstanceModel,
    t_ds_process_task_relation::Model as DsProcessTaskRelationModel,
    t_ds_process_task_relation_log::Model as DsProcessTaskRelationLogModel,
    t_ds_project::Model as DsProjectModel,
    t_ds_queue::Model as DsQueueModel,
    t_ds_relation_datasource_user::Model as DsRelationDatasourceUserModel,
    t_ds_relation_namespace_user::Model as DsRelationNamespaceUserModel,
    t_ds_relation_process_instance::Model as DsRelationProcessInstanceModel,
    t_ds_relation_project_user::Model as DsRelationProjectUserModel,
    t_ds_relation_resources_user::Model as DsRelationResourcesUserModel,
    t_ds_relation_rule_execute_sql::Model as DsRelationRuleExecuteSqlModel,
    t_ds_relation_rule_input_entry::Model as DsRelationRuleInputEntryModel,
    t_ds_relation_udfs_user::Model as DsRelationUdfsUserModel,
    t_ds_resources::Model as DsResourcesModel,
    t_ds_schedules::Model as DsSchedulesModel,
    t_ds_session::Model as DsSessionModel,
    t_ds_task_definition::Model as DsTaskDefinitionModel,
    t_ds_task_definition_log::Model as DsTaskDefinitionLogModel,
    t_ds_task_group::Model as DsTaskGroupModel,
    t_ds_task_group_queue::Model as DsTaskGroupQueueModel,
    t_ds_task_instance::Model as DsTaskInstanceModel,
    t_ds_tenant::Model as DsTenantModel,
    t_ds_udfs::Model as DsUdfsModel,
    t_ds_user::Model as DsUserModel,
    t_ds_version::Model as DsVersionModel,
    t_ds_worker_group::Model as DsWorkerGroupModel,
};

use proto::{
    ds_access_token::DsAccessTokenBean,
    ds_alert::DsAlertBean,
    ds_alert_plugin_instance::DsAlertPluginInstanceBean,
    ds_alert_send_status::DsAlertSendStatusBean,
    ds_alertgroup::DsAlertGroupBean,
    ds_audit_log::DsAuditLogBean,
    ds_command::DsCommandBean,
    ds_datasource::DsDatasourceBean,
    ds_dq_comparison_type::DsDqComparisonTypeBean,
    ds_dq_execute_result::DsDqExecuteResultBean,
    ds_dq_rule::DsDqRuleBean,
    ds_dq_rule_execute_sql::DsDqRuleExecuteSqlBean,
    ds_dq_rule_input_entry::DsDqRuleInputEntryBean,
    ds_dq_task_statistics_value::DsDqTaskStatisticsValueBean,
    ds_environment::DsEnvironmentBean,
    ds_environment_worker_group_relation::DsEnvironmentWorkerGroupRelationBean,
    ds_error_command::DsErrorCommandBean,
    ds_k8s::DsK8sBean,
    ds_k8s_namespace::DsK8sNamespaceBean,
    ds_plugin_define::DsPluginDefineBean,
    ds_process_definition::DsProcessDefinitionBean,
    ds_process_definition_log::DsProcessDefinitionLogBean,
    ds_process_instance::DsProcessInstanceBean,
    ds_process_task_relation::DsProcessTaskRelationBean,
    ds_process_task_relation_log::DsProcessTaskRelationLogBean,
    ds_project::DsProjectBean,
    ds_queue::DsQueueBean,
    ds_relation_datasource_user::DsRelationDatasourceUserBean,
    ds_relation_namespace_user::DsRelationNamespaceUserBean,
    ds_relation_process_instance::DsRelationProcessInstanceBean,
    ds_relation_project_user::DsRelationProjectUserBean,
    ds_relation_resources_user::DsRelationResourcesUserBean,
    ds_relation_rule_execute_sql::DsRelationRuleExecuteSqlBean,
    ds_relation_rule_input_entry::DsRelationRuleInputEntryBean,
    ds_relation_udfs_user::DsRelationUdfsUserBean,
    ds_resources::DsResourceBean,
    ds_schedules::DsSchedulesBean,
    ds_session::DsSessionBean,
    ds_task_definition::DsTaskDefinitionBean,
    ds_task_definition_log::DsTaskDefinitionLogBean,
    ds_task_group::DsTaskGroupBean,
    ds_task_group_queue::DsTaskGroupQueueBean,
    ds_task_instance::DsTaskInstanceBean,
    ds_tenant::DsTenantBean,
    ds_udfs::DsUdfsBean,
    ds_user::DsUserBean,
    ds_version::DsVersionBean,
    ds_worker_group::DsWorkerGroupBean,
    qrtz_blob_triggers::QrtzBlobTriggerBean,
    qrtz_calendars::QrtzCalendarBean,
    qrtz_cron_triggers::QrtzCronTriggersBean,
    qrtz_fired_triggers::QrtzFiredTriggersBean,
    qrtz_job_details::QrtzJobDetailsBean,
    qrtz_locks::QrtzLocksBean,
    qrtz_paused_trigger_grps::QrtzPausedTriggerGrpsBean,
    qrtz_scheduler_state::QrtzSchedulerStateBean,
    qrtz_simple_triggers::QrtzSimpleTriggerBean,
    qrtz_simprop_triggers::QrtzSimpropTriggerBean,
    qrtz_triggers::QrtzTriggerBean,
};


macro_rules! impl_from {
    ($from:ty, $to:ty) => {
        impl From<$from> for $to {
            fn from(model: $from) -> Self {
                serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap()
            }
        }
    };
}


impl_from!(DsAccessTokenModel, DsAccessTokenBean);
impl_from!(DsAlertModel, DsAlertBean);
impl_from!(DsAlertPluginInstanceModel, DsAlertPluginInstanceBean);
impl_from!(DsAlertSendStatusModel, DsAlertSendStatusBean);
impl_from!(DsAlertgroupModel, DsAlertGroupBean);
impl_from!(DsAuditLogModel, DsAuditLogBean);
impl_from!(DsCommandModel, DsCommandBean);
impl_from!(DsDatasourceModel, DsDatasourceBean);
impl_from!(DsDqComparisonTypeModel, DsDqComparisonTypeBean);
impl_from!(DsDqExecuteResultModel, DsDqExecuteResultBean);
impl_from!(DsDqRuleModel, DsDqRuleBean);
impl_from!(DsDqRuleExecuteSqlModel, DsDqRuleExecuteSqlBean);
impl_from!(DsDqRuleInputEntryModel, DsDqRuleInputEntryBean);
impl_from!(DsDqTaskStatisticsValueModel, DsDqTaskStatisticsValueBean);
impl_from!(DsEnvironmentModel, DsEnvironmentBean);
impl_from!(
    DsEnvironmentWorkerGroupRelationModel,
    DsEnvironmentWorkerGroupRelationBean
);
impl_from!(DsErrorCommandModel, DsErrorCommandBean);
impl_from!(DsK8sModel, DsK8sBean);
impl_from!(DsK8sNamespaceModel, DsK8sNamespaceBean);
impl_from!(DsPluginDefineModel, DsPluginDefineBean);
impl_from!(DsProcessDefinitionModel, DsProcessDefinitionBean);
impl_from!(DsProcessDefinitionLogModel, DsProcessDefinitionLogBean);
impl_from!(DsProcessInstanceModel, DsProcessInstanceBean);
impl_from!(DsProcessTaskRelationModel, DsProcessTaskRelationBean);
impl_from!(DsProcessTaskRelationLogModel, DsProcessTaskRelationLogBean);
impl_from!(DsProjectModel, DsProjectBean);
impl_from!(DsQueueModel, DsQueueBean);
impl_from!(DsRelationDatasourceUserModel, DsRelationDatasourceUserBean);
impl_from!(DsRelationNamespaceUserModel, DsRelationNamespaceUserBean);
impl_from!(
    DsRelationProcessInstanceModel,
    DsRelationProcessInstanceBean
);
impl_from!(DsRelationProjectUserModel, DsRelationProjectUserBean);
impl_from!(DsRelationResourcesUserModel, DsRelationResourcesUserBean);
impl_from!(DsRelationRuleExecuteSqlModel, DsRelationRuleExecuteSqlBean);
impl_from!(DsRelationRuleInputEntryModel, DsRelationRuleInputEntryBean);
impl_from!(DsRelationUdfsUserModel, DsRelationUdfsUserBean);
impl_from!(DsResourcesModel, DsResourceBean);
impl_from!(DsSchedulesModel, DsSchedulesBean);
impl_from!(DsSessionModel, DsSessionBean);
impl_from!(DsTaskDefinitionModel, DsTaskDefinitionBean);
impl_from!(DsTaskDefinitionLogModel, DsTaskDefinitionLogBean);
impl_from!(DsTaskGroupModel, DsTaskGroupBean);
impl_from!(DsTaskGroupQueueModel, DsTaskGroupQueueBean);
impl_from!(DsTaskInstanceModel, DsTaskInstanceBean);
impl_from!(DsTenantModel, DsTenantBean);
impl_from!(DsUdfsModel, DsUdfsBean);
impl_from!(DsUserModel, DsUserBean);
impl_from!(DsVersionModel, DsVersionBean);
impl_from!(DsWorkerGroupModel, DsWorkerGroupBean);
impl_from!(QrtzBlobTriggerModel, QrtzBlobTriggerBean);
impl_from!(QrtzCalendarModel, QrtzCalendarBean);
impl_from!(QrtzCronTriggersModel, QrtzCronTriggersBean);
impl_from!(QrtzFiredTriggersModel, QrtzFiredTriggersBean);
impl_from!(QrtzJobDetailsModel, QrtzJobDetailsBean);
impl_from!(QrtzLocksModel, QrtzLocksBean);
impl_from!(QrtzPausedTriggerGrpsModel, QrtzPausedTriggerGrpsBean);
impl_from!(QrtzSchedulerStateModel, QrtzSchedulerStateBean);
impl_from!(QrtzSimpleTriggersModel, QrtzSimpleTriggerBean);
impl_from!(QrtzTriggersModel, QrtzTriggerBean);
impl_from!(QrtzSimpropTriggerModel, QrtzSimpropTriggerBean);


// impl From<DsUserModel> for DsUserBean}; {
//     fn from(model: DsUserModel) -> Self {
//         serde_json::from_str(&serde_json::to_string(&model).unwrap()).unwrap()

//         // Self {
//         //     id: model.id,
//         //     user_name: model.user_name,
//         //     user_password: model.user_password,
//         //     user_type: model.user_type,
//         //     email: model.email,
//         //     phone: model.phone,
//         //     tenant_id: model.tenant_id,
//         //     create_time: model.create_time.map(|x| x.to_string()),
//         //     update_time: model.update_time.map(|x| x.to_string()),
//         //     queue: model.queue,
//         //     state: model.state,
//         //     time_zone: model.time_zone,
//         // }
//     }
// }


// impl From<crate::qrtz_blob_triggers::Model> for QrtzBlobTriggerBean}; {
//     fn from(model: crate::qrtz_blob_triggers::Model) -> Self {
//         Self {
//             sched_name: model.sched_name,
//             trigger_name: model.trigger_name,
//             trigger_group: model.trigger_group,
//             blob_data: model.blob_data,
//         }
//     }
// }
