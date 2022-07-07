use std::{env, fs};
use std::path::Path;

use flapigen::{JavaConfig, LanguageConfig};
use protobuf_codegen::Codegen;
use protoc_bin_vendored::protoc_bin_path;
use rifgen::{Generator, Language, TypeCases};

fn main() {
    proto_buffer();
    jni_bridge();
}

fn proto_buffer() {
    Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_path().unwrap())
        .includes(&["../proto"])
        .input("../proto/messages.proto")
        .cargo_out_dir("protos")
        .include("protos")
        .run_from_script();
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy(format!("{}/protos/messages.rs", out_dir), "src/messages.rs").unwrap();
}

fn jni_bridge() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let in_src = "src/java_glue.rs.in";
    let out_src = Path::new(&out_dir).join("java_glue.rs");
    Generator::new(TypeCases::CamelCase, Language::Java, "src").generate_interface(in_src);
    let java_folder = Path::new("../android/app/src/main/java/ademar/rai/lib");
    if java_folder.exists() {
        fs::remove_dir_all(java_folder).unwrap();
    }
    fs::create_dir(java_folder).unwrap();
    let swig_gen = flapigen::Generator::new(LanguageConfig::JavaConfig(
        JavaConfig::new(java_folder.into(), "ademar.rai.lib".into())
            .use_null_annotation_from_package("androidx.annotation".into()),
    )).rustfmt_bindings(true).merge_type_map("additional_glue", include_str!("src/additional_glue.rs"));
    swig_gen.expand("android bindings", &in_src, &out_src);
    println!("cargo:rerun-if-changed=src");
}
