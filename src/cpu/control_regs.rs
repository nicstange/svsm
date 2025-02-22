// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

use super::features::cpu_has_pge;
use bitflags::bitflags;
use core::arch::asm;

pub fn cr0_init() {
    let mut cr0 = read_cr0();

    cr0.insert(CR0Flags::WP); // Enable Write Protection
    cr0.remove(CR0Flags::NW); // Enable caches ...
    cr0.remove(CR0Flags::CD); // ... if not already happened

    write_cr0(cr0);
}

pub fn cr4_init() {
    let mut cr4 = read_cr4();

    cr4.insert(CR4Flags::PSE); // Enable Page Size Extensions

    if cpu_has_pge() {
        cr4.insert(CR4Flags::PGE); // Enable Global Pages
    }

    write_cr4(cr4);
}

bitflags! {
    pub struct CR0Flags: u64 {
        const PE = 1 << 0;  // Protection Enabled
        const MP = 1 << 1;  // Monitor Coprocessor
        const EM = 1 << 2;  // Emulation
        const TS = 1 << 3;  // Task Switched
        const ET = 1 << 4;  // Extension Type
        const NE = 1 << 5;  // Numeric Error
        const WP = 1 << 16; // Write Protect
        const AM = 1 << 18; // Alignment Mask
        const NW = 1 << 29; // Not Writethrough
        const CD = 1 << 30; // Cache Disable
        const PG = 1 << 31; // Paging
    }
}

pub fn read_cr0() -> CR0Flags {
    let cr0: u64;

    unsafe {
        asm!("mov %cr0, %rax",
             out("rax") cr0,
             options(att_syntax));
    }

    CR0Flags::from_bits_truncate(cr0)
}

pub fn write_cr0(cr0: CR0Flags) {
    let reg = cr0.bits();

    unsafe {
        asm!("mov %rax, %cr0",
             in("rax") reg,
             options(att_syntax));
    }
}

pub fn read_cr2() -> usize {
    let ret: usize;
    unsafe {
        asm!("mov %cr2, %rax",
             out("rax") ret,
             options(att_syntax));
    }
    ret
}

pub fn write_cr2(cr2: usize) {
    unsafe {
        asm!("mov %rax, %cr2",
             in("rax") cr2,
             options(att_syntax));
    }
}

pub fn read_cr3() -> usize {
    let ret: usize;
    unsafe {
        asm!("mov %cr3, %rax",
             out("rax") ret,
             options(att_syntax));
    }
    ret
}

pub fn write_cr3(cr3: usize) {
    unsafe {
        asm!("mov %rax, %cr3",
             in("rax") cr3,
             options(att_syntax));
    }
}

bitflags! {
    pub struct CR4Flags: u64 {
        const VME       = 1 << 0;  // Virtual-8086 Mode Extensions
        const PVI       = 1 << 1;  // Protected-Mode Virtual Interrupts
        const TSD       = 1 << 2;  // Time Stamp Disable
        const DE        = 1 << 3;  // Debugging Extensions
        const PSE       = 1 << 4;  // Page Size Extensions
        const PAE       = 1 << 5;  // Physical-Address Extension
        const MCE       = 1 << 6;  // Machine Check Enable
        const PGE       = 1 << 7;  // Page-Global Enable
        const PCE       = 1 << 8;  // Performance-Monitoring Counter Enable
        const OSFXSR        = 1 << 9;  // Operating System FXSAVE/FXRSTOR Support
        const OSXMMEXCPT    = 1 << 10; // Operating System Unmasked Exception Support
        const UMIP      = 1 << 11; // User Mode Instruction Prevention
        const FSGSBASE      = 1 << 16; // Enable RDFSBASE, RDGSBASE, WRFSBASE, and
                           // WRGSBASE instructions
        const PCIDE     = 1 << 17; // Process Context Identifier Enable
        const OSXSAVE       = 1 << 18; // XSAVE and Processor Extended States Enable Bit
        const SMEP      = 1 << 20; // Supervisor Mode Execution Prevention
        const SMAP      = 1 << 21; // Supervisor Mode Access Protection
        const PKE       = 1 << 22; // Protection Key Enable
        const CET       = 1 << 23; // Control-flow Enforcement Technology
    }
}

pub fn read_cr4() -> CR4Flags {
    let cr4: u64;

    unsafe {
        asm!("mov %cr4, %rax",
             out("rax") cr4,
             options(att_syntax));
    }

    CR4Flags::from_bits_truncate(cr4)
}

pub fn write_cr4(cr4: CR4Flags) {
    let reg = cr4.bits();

    unsafe {
        asm!("mov %rax, %cr4",
             in("rax") reg,
             options(att_syntax));
    }
}
