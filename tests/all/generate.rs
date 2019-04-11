use assert_cmd::prelude::*;
use std::str;
use utils;

#[test]
fn generate_with_defaults() {
    let fixture = utils::fixture::not_a_crate();
    let cmd = fixture.wasm_pack().arg("generate").assert().failure();

    let output = cmd.get_output();

    assert!(str::from_utf8(&output.stdout)
        .unwrap()
        .contains("hello-wasm"));
}

#[test]
fn generate_with_provided_name() {
    let fixture = utils::fixture::not_a_crate();
    let cmd = fixture
        .wasm_pack()
        .arg("generate")
        .arg("--name ferris")
        .assert()
        .failure();

    let output = cmd.get_output();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("ferris"));
}
