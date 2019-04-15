use assert_cmd::prelude::*;
use std::str;
use utils;

#[test]
fn generate_with_defaults() {
    let fixture = utils::fixture::not_a_crate();
    let cmd = fixture.wasm_pack().arg("generate").assert().failure();
}

#[test]
fn generate_with_provided_name() {
    let fixture = utils::fixture::not_a_crate();
    let cmd = fixture
        .wasm_pack()
        .arg("generate")
        .arg("--name")
        .arg("ferris")
        .assert();

    let output = cmd.get_output();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("ferris"));
}
