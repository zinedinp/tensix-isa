//! Tensix tile memory map and configuration registers for Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`tensix`] | Tile memory map, baby RISC-V cores (`Core`), reset PCs, soft reset |
//! | [`cfg`](mod@cfg) | Backend-configuration fields; `cfg::generated` comes from `cfg_defines.h` (`cargo xtask gen-cfg`) |
//!
//! `no_std`, no dependencies.
#![no_std]
#![forbid(unsafe_code)]

pub mod cfg;
pub mod tensix;
