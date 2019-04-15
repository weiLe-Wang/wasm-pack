use assert_cmd::prelude::*;
use std::str;
use utils;

#[test]
fn new_with_no_name_errors() {
    let fixture = utils::fixture::not_a_crate();
    fixture.wasm_pack().arg("new").assert().failure();
}

#[test]
fn new_with_provided_name() {
    let fixture = utils::fixture::not_a_crate();
    let cmd = fixture
        .wasm_pack()
        .arg("new")
        .arg("--name")
        .arg("ferris")
        .assert();

    let output = cmd.get_output();

    assert!(str::from_utf8(&output.stdout).unwrap().contains("ferris"));
}
