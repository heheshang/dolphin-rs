fn main() {
    let proto_file = "./proto/t_ds_user.proto";


    tonic_build::configure()
    // .compile_with_config(config, protos, includes)
        .build_server(true)
        // .out_dir("./src/pb")
        .compile(&[proto_file], &["."])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    println!("cargo:rerun-if-changed={}", proto_file);
}
