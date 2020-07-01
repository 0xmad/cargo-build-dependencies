use clap::{App, Arg};

use std::env;
use std::process::Command;

mod package;

fn main() {
    let matched_args = App::new("cargo build-dependencies")
        .arg(Arg::with_name("build-dependencies"))
        .arg(Arg::with_name("release").long("release"))
        .arg(Arg::with_name("target").long("target"))
        .get_matches();

    let is_release = matched_args.is_present("release");
    let target = match matched_args.value_of("target") {
        Some(value) => value,
        None => "",
    };

    let cargo_toml = package::get_toml("Cargo.toml").expect("Can't get Cargo.toml");
    let cargo_lock = package::get_toml("Cargo.lock").expect("Can't get Cargo.lock");
    let dependencies = package::get_dependencies(&cargo_toml, &cargo_lock);

    if dependencies.is_empty() {
        panic!("Can't find dependencies");
    }

    println!("Start building packages");

    for dependency in dependencies {
        build_package(&dependency, is_release, &target);
    }

    println!("Finished");
}

fn build_package(pkg_name: &str, is_release: bool, target: &str) {
    println!("Building package: {:?}", pkg_name);

    let mut command = Command::new("cargo");
    let command_with_args = command.arg("build").arg("-p").arg(pkg_name);

    let command_with_args = if is_release {
        command_with_args.arg("--release")
    } else {
        command_with_args
    };

    let command_with_args = if !target.is_empty() {
        command_with_args.arg("--target=".to_string() + target)
    } else {
        command_with_args
    };

    execute_command(command_with_args);
}

fn execute_command(command: &mut Command) {
    let mut child = command
        .envs(env::vars())
        .spawn()
        .expect("Failed to execute process");

    let exit_status = child.wait().expect("Failed to run command");

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("Exited with status code: {}", code),
            None => panic!("Process terminated by signal"),
        }
    }
}
