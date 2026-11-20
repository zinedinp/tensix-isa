//! Tenstorrent Blackhole hardware description, the `tensix-isa` layer in one crate.
//!
//! Re-exports every crate of the layer under its module name:
//!
//! | Module | Crate | Contents |
//! |---|---|---|
//! | [`isa`], [`adc`], [`frontend`] | `tensix-insn` | Instruction encoders, address counters, MOP expander |
//! | [`tensix`], [`cfg`](mod@cfg) | `tensix-regs` | Tile memory map, cores, configuration registers |
//! | [`noc`], [`dram`], [`arc`], [`eth`] | `tensix-noc` | NoC coordinates, GDDR channels, ARC telemetry, Ethernet tiles |
//! | [`numerics`] | `tensix-numerics` | Bit-exact arithmetic models |
//! | [`tile`], [`format`] | `tensix-tile` | Tile layout and data formats |
//! | [`backend`], [`sfpu`], [`matrix`], [`packer`], [`unpacker`], [`sync`], [`scalar`], [`hazard`], [`tag_search`] | `tensix-units` | Checked unit builders |
//! | [`l1`], [`mailbox`], [`dm`], [`dataflow`], [`ttinsn`] | `tensix-fw-abi` | Host/firmware protocol |
//!
//! `tensix-layout` (host tilization, needs `std`) is a separate crate.
//!
//! Every constant cites the specification page it came from. The Blackhole documentation
//! tree is a delta over Wormhole's, so a fact sourced from a Wormhole page is marked
//! `UNVERIFIED` until measured. `no_std`, built for the host and for `riscv32im`.
#![no_std]
#![forbid(unsafe_code)]

pub use tensix_fw_abi::{dataflow, dm, l1, mailbox, ttinsn};
pub use tensix_insn::{adc, frontend, isa};
pub use tensix_noc::{arc, dram, eth, noc};
pub use tensix_numerics::numerics;
pub use tensix_regs::{cfg, tensix};
pub use tensix_tile::{format, tile};
pub use tensix_units::{backend, hazard, matrix, packer, scalar, sfpu, sync, tag_search, unpacker};

/// The tile descriptor is exactly the width of the configuration span that holds it: a
/// regenerated `cfg_defines.h` that disagreed would fail the build.
const _: () = assert!(
    cfg::generated::thcon::THCON_SEC0_REG0_TileDescriptor.words() as usize
        == tile::TileDescriptor::WORDS,
    "TileDescriptor is four Config words"
);
