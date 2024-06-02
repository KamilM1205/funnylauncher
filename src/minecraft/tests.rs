use std::fs;

use crate::minecraft::minecraft_json::{Library, MinecraftJson};

#[test]
fn load_forge_json_test() {
    let data = fs::read_to_string("tests_file/1.16.5-forge-36.2.34.json").unwrap();
    let mr = MinecraftJson::new(&data).unwrap();

    println!("{:?}", mr);
    println!("{:?}", serde_json::to_string(&mr).unwrap());
}

#[test]
fn load_minecraft_json_test() {
    let data = fs::read_to_string("tests_file/1.16.5.json").unwrap();
    let mr = MinecraftJson::new(&data).unwrap();

    println!("{:?}", mr);
    println!("{}", serde_json::to_string(&mr).unwrap());
}

#[test]
fn libs_to_args_test() {
    let mut mr = MinecraftJson::default();

    let mut d1 = Library::default();
    d1.downloads.artifact.path =
        "net/minecraftforge/forge/1.16.5-36.2.34/forge-1.16.5-36.2.34.jar".to_string();
    let mut d2 = Library::default();
    d2.downloads.artifact.path = "org/ow2/asm/asm/9.1/asm-9.1.jar".to_string();

    mr.libraries = vec![d1, d2];

    assert_eq!(mr.libs_to_args("~/"), "~/net/minecraftforge/forge/1.16.5-36.2.34/forge-1.16.5-36.2.34.jar;~/org/ow2/asm/asm/9.1/asm-9.1.jar;");
}

#[test]
fn jvm_args_to_arg_test() {
    let data = fs::read_to_string("tests_file/1.16.5-forge-36.2.34.json").unwrap();
    let mr = MinecraftJson::new(&data).unwrap();
    println!("{:?}", mr.jvm_args_to_arg());
}
