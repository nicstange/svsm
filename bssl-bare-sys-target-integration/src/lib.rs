//! Integrate bssl-bare-sys into SVSM.

#![no_std]

// Explicit dependency to make sure libcrt gets included in the final link.
use libcrt as _;
