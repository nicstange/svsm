// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 SUSE LLC
// Author: Nicolai Stange <nstange@suse.de>

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let top_src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let top_src_path = PathBuf::from(top_src_dir.clone());
    let libcrt_src_path = top_src_path.join("libcrt");
    let libcrt_src_dir = libcrt_src_path
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir.clone());

    let status = Command::new("make")
        .current_dir(out_path.clone())
        .args([
            format!(
                "-f{}",
                libcrt_src_path
                    .join("Makefile")
                    .into_os_string()
                    .into_string()
                    .unwrap()
            ),
            format!("VPATH={}", libcrt_src_dir),
            String::from("clean"),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(status.success());

    let status = Command::new("make")
        .current_dir(out_path)
        .args([
            format!("-f{}/libcrt/Makefile", top_src_dir),
            format!("VPATH={}/libcrt", top_src_dir),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
    assert!(status.success());

    // Define $DEP_LIBCRT_CPPFLAGS for dependant packages.
    println!(
        "cargo::metadata=CPPFLAGS=-I{}",
        libcrt_src_path
            .join("include")
            .into_os_string()
            .into_string()
            .unwrap()
    );
    // Define $DEP_LIBCRT_LINK_SEARCH for dependant packages.
    println!("cargo::metadata=LINK_SEARCH={}", out_dir);
    // Define $DEP_LIBCRT_LINK_LIB for dependant packages.
    println!("cargo::metadata=LINK_LIB=crt");

    println!("cargo::rustc-link-search={}", out_dir);
    println!("cargo::rustc-link-lib=crt");
}
