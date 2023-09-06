use std::process::Command;

use prost_build::Config;
use proto_builder_trait::prost::BuilderAttributes;

// use proto_builder_trait::prost::BuilderAttributes;

fn main() {
    // Config::default()
    //     .out_dir("src/pb")

    //     .with_serde(
    //         &["ds_user.DsUser", "ds_session.DsSession"],
    //         true,
    //         true,
    //         Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
    //         // None,
    //     )
    //     .with_derive_builder(
    //         &["ds_user.DsUser", "ds_session.DsSession"],
    //         Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    //     )
    //     // .with_sqlx_type(&["ds_user.DsUser", "ds_session.DsSession"], None)
    //     // .with_strum(
    //     //     &["ds_user.DsUser", "ds_session.DsSession"],
    //     //     Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
    //     // )
    //     //   .with_serde_as(
    //     //         "ds_user.DsUser",
    //     //         &[(
    //     //             &["update_time", "create_time"],
    //     //             r#"#[serde_as(as = "chrono::DateTime<chrono::Utc>")]"#,
    //     //         )],
    //     //     )
    //     // .with_field_attributes(&["ds_user.DsUser.create_time", "ds_user.DsUser.update_time"], &[
    //     //     "#[derive(Copy)]",
    //     // ])
    //     .compile_protos(&["protos/ds_session.proto","protos/ds_user.proto"], &["protos"])
    //     .unwrap();

    // let c = Config::default()
    //     .out_dir("src/pb")
    //     .with_serde(
    //         &["todo.Todo", "todo.TodoStatus"],
    //         true,
    //         true,
    //         Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
    //     )
    //     .with_serde_as("todo.Todo", &[(
    //         &["status", "created_at"],
    //         r#"#[serde_as(as = "DisplayFromStr")]"#,
    //     )])
    //     .with_derive_builder(
    //         &["todo.Todo"],
    //         Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    //     )
    //     // .with_sqlx_type(&["todo.TodoStatus"], None)
    //     .with_strum(
    //         &["todo.TodoStatus"],
    //         Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
    //     );
    // .with_field_attributes(&["todo.Todo.created_at", "todo.Todo.updated_at"], &[
    //     "#[derive(Copy)]",
    // ])
    //     .service_generator(ServiceGenerator::new())
    //      .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"]);

    // tonic_build::configure()
    // .compile_with_config(c, &["fixtures/protos/todo.proto"], &["fixtures/protos"])

    //     .unwrap();

    // let _ = prost_build::compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"]);
    // let _ = prost_build::compile_protos(&["protos/ds_session.proto"], &["protos"]);
    // let _ = prost_build::compile_protos(&["protos/ds_user.proto"], &["protos"]);

    let mut config = Config::default();
    config.with_serde(
        &[
            "ds_access_token.DsAccessTokenBean",
            "ds_alert_plugin_instance.DsAlertPluginInstanceBean",
            "ds_alert_send_status.DsAlertSendStatusBean",
            "ds_alert.DsAlertBean",
            "ds_alertgroup.DsAlertGroupBean",
            "ds_audit_log.DsAuditLogBean",
            "ds_command.DsCommandBean",
            "ds_datasource.DsDatasourceBean",
            "ds_dq_comparison_type.DsDqComparisonTypeBean",
            "ds_dq_execute_result.DsDqExecuteResultBean",
            "ds_dq_rule_execute_sql.DsDqRuleExecuteSqlBean",
            "ds_dq_rule_input_entry.DsDqRuleInputEntryBean",
            "ds_dq_rule.DsDqRuleBean",
            "ds_dq_task_statistics_value.DsDqTaskStatisticsValueBean",
            "ds_environment_worker_group_relation.DsEnvironmentWorkerGroupRelationBean",
            "ds_environment.DsEnvironmentBean",
            "ds_error_command.DsErrorCommandBean",
            "ds_k8s_namespace.DsK8sNamespaceBean",
            "ds_k8s.DsK8sBean",
            "ds_plugin_define.DsPluginDefineBean",
            "ds_process_definition_log.DsProcessDefinitionLogBean",
            "ds_process_definition.DsProcessDefinitionBean",
            "ds_process_instance.DsProcessInstanceBean",
            "ds_process_task_relation_log.DsProcessTaskRelationLogBean",
            "ds_process_task_relation.DsProcessTaskRelationBean",
            "ds_project.DsProjectBean",
            "ds_queue.DsQueueBean",
            "ds_relation_datasource_user.DsRelationDatasourceUserBean",
            "ds_relation_namespace_user.DsRelationNamespaceUserBean",
            "ds_relation_process_instance.DsRelationProcessInstanceBean",
            "ds_relation_project_user.DsRelationProjectUserBean",
            "ds_relation_resources_user.DsRelationResourcesUserBean",
            "ds_relation_rule_execute_sql.DsRelationRuleExecuteSqlBean",
            "ds_relation_rule_input_entry.DsRelationRuleInputEntryBean",
            "ds_relation_udfs_user.DsRelationUdfsUserBean",
            "ds_resources.DsResourceBean",
            "ds_schedules.DsSchedulesBean",
            "ds_session.DsSessionBean",
            "ds_task_definition_log.DsTaskDefinitionLogBean",
            "ds_task_definition.DsTaskDefinitionBean",
            "ds_task_group_queue.DsTaskGroupQueueBean",
            "ds_task_group.DsTaskGroupBean",
            "ds_task_instance.DsTaskInstanceBean",
            "ds_tenant.DsTenantBean",
            "ds_udfs.DsUdfsBean",
            "ds_user.DsUserBean",
            "ds_user.Flag",
            "ds_version.DsVersionBean",
            "ds_worker_group.DsWorkerGroupBean",
            "qrtz_blob_triggers.QrtzBlobTriggerBean",
            "qrtz_calendars.QrtzCalendarBean",
            "qrtz_cron_triggers.QrtzCronTriggersBean",
            "qrtz_fired_triggers.QrtzFiredTriggersBean",
            "qrtz_job_details.QrtzJobDetailsBean",
            "qrtz_locks.QrtzLocksBean",
            "qrtz_paused_trigger_grps.QrtzPausedTriggerGrpsBean",
            "qrtz_scheduler_state.QrtzSchedulerStateBean",
            "qrtz_simple_triggers.QrtzSimpleTriggerBean",
            "qrtz_simprop_triggers.QrtzSimpropTriggerBean",
            "qrtz_triggers.QrtzTriggerBean",
        ],
        true,
        true,
        Some(&[r#"#[serde(rename_all = "snake_case")]"#]),
        // None,
    );
    config.with_serde_as("ds_user.DsUser", &[(
        &["create_time", "update_time"],
        r#"#[serde_as(as = "DisplayFromStr")]"#,
    )]);
    config.with_derive_builder(
        &["ds_user.DsUser", "ds_session.DsSession"],
        Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
    );

    tonic_build::configure()
        .out_dir("src/pb")
        .compile_with_config(
            config,
            &[
                "protos/googleapis/google/ds/v1/qrtz_blob_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_calendars.proto",
                "protos/googleapis/google/ds/v1/qrtz_cron_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_fired_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_job_details.proto",
                "protos/googleapis/google/ds/v1/qrtz_locks.proto",
                "protos/googleapis/google/ds/v1/qrtz_paused_trigger_grps.proto",
                "protos/googleapis/google/ds/v1/qrtz_scheduler_state.proto",
                "protos/googleapis/google/ds/v1/qrtz_simple_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_simprop_triggers.proto",
                "protos/googleapis/google/ds/v1/qrtz_triggers.proto",
                "protos/googleapis/google/ds/v1/t_ds_access_token.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert_plugin_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_alert_send_status.proto",
                "protos/googleapis/google/ds/v1/t_ds_alertgroup.proto",
                "protos/googleapis/google/ds/v1/t_ds_audit_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_command.proto",
                "protos/googleapis/google/ds/v1/t_ds_datasource.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_comparison_type.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_execute_result.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule_execute_sql.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_rule_input_entry.proto",
                "protos/googleapis/google/ds/v1/t_ds_dq_task_statistics_value.proto",
                "protos/googleapis/google/ds/v1/t_ds_environment.proto",
                "protos/googleapis/google/ds/v1/t_ds_environment_worker_group_relation.proto",
                "protos/googleapis/google/ds/v1/t_ds_error_command.proto",
                "protos/googleapis/google/ds/v1/t_ds_k8s.proto",
                "protos/googleapis/google/ds/v1/t_ds_k8s_namespace.proto",
                "protos/googleapis/google/ds/v1/t_ds_plugin_define.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_definition.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_definition_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_task_relation.proto",
                "protos/googleapis/google/ds/v1/t_ds_process_task_relation_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_project.proto",
                "protos/googleapis/google/ds/v1/t_ds_queue.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_datasource_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_namespace_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_process_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_project_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_resources_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_rule_execute_sql.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_rule_input_entry.proto",
                "protos/googleapis/google/ds/v1/t_ds_relation_udfs_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_resources.proto",
                "protos/googleapis/google/ds/v1/t_ds_schedules.proto",
                "protos/googleapis/google/ds/v1/t_ds_session.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_definition.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_definition_log.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_group.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_group_queue.proto",
                "protos/googleapis/google/ds/v1/t_ds_task_instance.proto",
                "protos/googleapis/google/ds/v1/t_ds_tenant.proto",
                "protos/googleapis/google/ds/v1/t_ds_udfs.proto",
                "protos/googleapis/google/ds/v1/t_ds_user.proto",
                "protos/googleapis/google/ds/v1/t_ds_version.proto",
                "protos/googleapis/google/ds/v1/t_ds_worker_group.proto",
            ],
            &["protos/googleapis"],
        )
        .unwrap();
    //    tonic_build::configure()
    //       .build_server(false)
    //       .out_dir("src/pb")
    //       .compile(&["protos/googleapis/google/pubsub/v1/pubsub.proto"], &[
    //           "protos/googleapis",
    //       ])
    //       .unwrap();
    println!("cargo:rerun-if-changed=protos/googleapis/google/pubsub/v1/pubsub.proto");
    println!("cargo:rerun-if-changed=protos/googleapis/google/api/annotations.proto");
    println!("cargo:rerun-if-changed=protos/googleapis/google/api/http.proto");
    println!("cargo:rerun-if-changed=protos/googleapis/google/api/field_behavior.proto");
    println!("cargo:rerun-if-changed=protos/googleapis/google/api/resource.proto");
    println!("cargo:rerun-if-changed=protos/googleapis/google/api/client.proto");
    println!("cargo:rerun-if-changed=protos/ds_session.proto");
    println!("cargo:rerun-if-changed=protos/ds_user.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_blob_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_calendars.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_cron_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_fired_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_job_details.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_locks.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_paused_trigger_grps.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_scheduler_state.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_simple_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_simprop_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/qrtz_triggers.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_access_token.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_alert.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_alert_plugin_instance.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_alert_send_status.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_alertgroup.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_audit_log.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_command.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_datasource.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_comparison_type.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_execute_result.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_rule.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_rule_execute_sql.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_rule_input_entry.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_dq_task_statistics_value.\
         proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_environment.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/\
         t_ds_environment_worker_group_relation.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_error_command.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_k8s.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_k8s_namespace.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_plugin_define.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_process_definition.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_process_definition_log.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_process_instance.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_process_task_relation.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_process_task_relation_log.\
         proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_project.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_queue.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_datasource_user.\
         proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_namespace_user.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_process_instance.\
         proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_project_user.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_resources_user.proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_rule_execute_sql.\
         proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_rule_input_entry.\
         proto"
    );
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_relation_udfs_user.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_resources.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_schedules.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_session.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_task_definition.proto");
    println!(
        "cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_task_definition_log.proto"
    );
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_task_group.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_task_group_queue.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_task_instance.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_tenant.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_udfs.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_user.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_version.proto");
    println!("cargo:return-if-changed=protos/googleapis/google/ds/v1/t_ds_worker_group.proto");
    Command::new("cargo").args(["fmt"]).output().unwrap();
}
