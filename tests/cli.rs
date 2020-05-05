use predicates::str::contains;
use std::process::Command;
use std::sync::mpsc;
use std::time;

use assert_cmd::prelude::*;

#[test]
fn client_cli_connect_server() {
    let (sender, receiver) = mpsc::sync_channel(0);
    let mut server = Command::cargo_bin("server").unwrap();
    let mut server_process = server.spawn().unwrap();
    let handle = std::thread::spawn(move || {
        let _ = receiver.recv();
        server_process.kill().expect("server died before killed");
    });
    // Sleep a bit so server can start
    std::thread::sleep(time::Duration::from_secs(1));

    // `client` with no args should exit with a non-zero code.
    Command::cargo_bin("client").unwrap().assert().failure();

    // `client` with list should sucessfully run. Default store is BERLIN_DE
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list"])
        .assert()
        .success()
        .stdout(contains("Products available on BERLIN_DE"));

    // `client` with list accepts store VENEZA_IT
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list", "--store", "VENEZA_IT"])
        .assert()
        .success()
        .stdout(contains("Products available on VENEZA_IT"));

    // `client` with list accepts -s
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list", "-s", "VENEZA_IT"])
        .assert()
        .success()
        .stdout(contains("Products available on VENEZA_IT"));

    // `client` with list accepts --show-unavailable
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list", "--show-unavailable"])
        .assert()
        .success()
        .stdout(contains("UNAVAILABLE"));

    // kill server
    sender.send(()).unwrap();
    handle.join().unwrap();

    // Start server again with different port
    let (sender, receiver) = mpsc::sync_channel(0);
    let mut server = Command::cargo_bin("server").unwrap();
    let mut server_process = server.args(&["--addr", "127.0.0.1:8080"]).spawn().unwrap();
    let handle = std::thread::spawn(move || {
        let _ = receiver.recv();
        server_process.kill().expect("server died before killed");
    });
    // Sleep a bit so server can start
    std::thread::sleep(time::Duration::from_secs(1));

    // `client` default port is not 8080
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list"])
        .assert()
        .failure();

    // `client` successfuly changes port
    Command::cargo_bin("client")
        .unwrap()
        .args(&["list", "--port", "8080"])
        .assert()
        .success();

    // kill server
    sender.send(()).unwrap();
    handle.join().unwrap();
}
