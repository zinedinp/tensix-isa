//! Checked builders for the Tensix units of Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`backend`] | Checked config reads, mutations and writes; unit waits |
//! | [`sfpu`] | SFPU instructions, hazards, `SFPLOADMACRO` configuration |
//! | [`matrix`] | `SrcA`/`SrcB` bank ownership, `MOVDBGA2D`/`GATESRCRST` |
//! | [`packer`], [`unpacker`] | Packer ReLU and edge masking; unpacker tileize and transpose |
//! | [`sync`] | Tensix semaphores and mutexes |
//! | [`scalar`] | Thread-local scalar arithmetic, L1 atomics, allowlisted MMIO |
//! | [`hazard`] | Backend wait rules and a checker for missing and redundant waits |
//! | [`tag_search`] | The L1 tag-search accelerator |
//!
//! `no_std`.
#![no_std]
#![forbid(unsafe_code)]

pub mod backend;
pub mod hazard;
pub mod matrix;
pub mod packer;
pub mod scalar;
pub mod sfpu;
pub mod sync;
pub mod tag_search;
pub mod unpacker;

use tensix_insn::isa;
use tensix_numerics::numerics;
use tensix_regs::{cfg, tensix};
