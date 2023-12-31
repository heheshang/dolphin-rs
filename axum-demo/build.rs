use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        // .build_server(false)
        // .out_dir("src/pb")
        .compile(&["proto/hello_world.proto"], &["/proto"])

        .unwrap();
}
