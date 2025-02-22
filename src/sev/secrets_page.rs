// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

use crate::types::VirtAddr;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct SecretsPage {
    pub version: u32,
    pub gctxt: u32,
    pub fms: u32,
    reserved_00c: u32,
    pub gosvw: [u8; 16],
    pub vmpck0: [u8; 32],
    pub vmpck1: [u8; 32],
    pub vmpck2: [u8; 32],
    pub vmpck3: [u8; 32],
    reserved_0a0: [u8; 96],
    pub vmsa_tweak_bmp: [u64; 8],
    pub svsm_base: u64,
    pub svsm_size: u64,
    pub svsm_caa: u64,
    pub svsm_max_version: u32,
    pub svsm_guest_vmpl: u8,
    reserved_15d: [u8; 3],
    pub tsc_factor: u32,
    reserved_164: [u8; 3740],
}

pub fn copy_secrets_page(target: &mut SecretsPage, source: VirtAddr) {
    let table = source as *const SecretsPage;

    unsafe {
        *target = *table;
    }
}
