// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_SIZE_2M: usize = PAGE_SIZE * 512;

#[allow(clippy::identity_op)]
pub const SVSM_CS: u16 = 1 * 8;
pub const SVSM_DS: u16 = 2 * 8;
pub const SVSM_USER_CS: u16 = 3 * 8;
pub const SVSM_USER_DS: u16 = 4 * 8;
pub const SVSM_TSS: u16 = 6 * 8;

pub const SVSM_CS_FLAGS: u16 = 0x29b;
pub const SVSM_DS_FLAGS: u16 = 0xc93;
pub const SVSM_TR_FLAGS: u16 = 0x89;

pub type PhysAddr = usize;
pub type VirtAddr = usize;

pub const MAX_CPUS: usize = 512;
