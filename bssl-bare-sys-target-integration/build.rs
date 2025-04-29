// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 SUSE LLC
// Author: Nicolai Stange <nstange@suse.de>

//! BoringSSL resp. bssl-bare-sys integration into SVSM.

use std::env;

fn main() {
    let _libcrt_cppflags = env::var("DEP_LIBCRT_CPPFLAGS").unwrap();
    let libcrt_pthread_cppflags = env::var("DEP_LIBCRT_PTHREAD_CPPFLAGS").unwrap();
    let libcrt_link_search = env::var("DEP_LIBCRT_LINK_SEARCH").unwrap();
    let libcrt_link_lib = env::var("DEP_LIBCRT_LINK_LIB").unwrap();

    // The non-thread CPPFLAGS don't get included here on purpose: BoringSSL's libcrypto needs a C++
    // STL, and we don't have one, so use the one from the host, which needs the libc from the host
    // as well. However, we do sneak the libcrt pthread headers in, so that BoringSSL types
    // containing e.g. locks will have the correct layout. For that to not clash with definitions
    // from the host libc, define its header guard.
    let integration_cppflags = format!(
        "{} \
         -D_BITS_PTHREADTYPES_COMMON_H \
         -DOPENSSL_THREADS \
         -DOPENSSL_NANOLIBC \
         -DOPENSSL_DOES_NOT_FORK \
         -DOPENSSL_NO_ASM",
        libcrt_pthread_cppflags
    );
    println!("cargo::metadata=CPPFLAGS={}", integration_cppflags);
    println!(
        "cargo::metadata=BINDGEN_CFLAGS={} --target=x86_64-none-linux",
        integration_cppflags
    );
    println!("cargo::metadata=CFLAGS=-fPIC -mcmodel=large -mno-mmx -mno-sse -mno-sse2 -mno-sse3 -mno-ssse3 -mno-sse4 -mno-avx -mno-avx2");
    println!("cargo::metadata=CXXFLAGS=-fPIC -mcmodel=large -mno-mmx -mno-sse -mno-sse2 -mno-sse3 -mno-ssse3 -mno-sse4 -mno-avx -mno-avx2");
    println!("cargo::metadata=ASLAGS=-fPIC");
    println!("cargo::metadata=LINK_SEARCH={}", libcrt_link_search);
    println!("cargo::metadata=LINK_LIB={}", libcrt_link_lib);
}
