fn main() {
    tauri_build::build()
    // prost_build::Config::new()
    //     .type_attribute(".event", "#[derive(serde::Serialize)]\n")
    //     .type_attribute(".event", "#[serde(rename_all = \"camelCase\")]")
    //     .extern_path(".model.server.v1", "crate::model")
    //     .out_dir("gen/proto")
    //     .compile_protos(
    //         &[
    //             "proto/model/server/v1/server.proto",
    //             "proto/event/server/v1/server.proto",
    //             "proto/event/topic/v1/topic.proto",
    //         ],
    //         &["proto"],
    //     )
    //     .expect("Failed to compile protos")
}
