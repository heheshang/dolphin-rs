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
        &["ds_user.DsUser", "ds_session.DsSession"],
        true,
        true,
        Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
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
            &["protos/ds_session.proto", "protos/ds_user.proto"],
            &["protos"],
        )
        .unwrap();
    println!("cargo:rerun-if-changed=protos/ds_session.proto");
    println!("cargo:rerun-if-changed=protos/ds_user.proto");
    Command::new("cargo").args(["fmt"]).output().unwrap();
}
