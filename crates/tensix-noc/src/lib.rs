//! Chip address model of Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`noc`] | NoC coordinate spaces (typed per NoC), NIU registers, the Tensix grid, NoC atomics, multicast rectangles, the probe protocol |
//! | [`dram`] | GDDR6 channels (`Dram`, `DramRange`) |
//! | [`arc`] | ARC telemetry tags (harvesting, translation) |
//! | [`eth`] | Ethernet tile register map, firmware-owned L1, the `eth::mover` protocol |
//!
//! `no_std`, no dependencies.
#![no_std]
#![forbid(unsafe_code)]

pub mod arc;
pub mod dram;
pub mod eth;
pub mod noc;
