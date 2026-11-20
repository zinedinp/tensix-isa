//! Tile layout and data formats of Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`tile`] | Tile layout in L1 (`TileDescriptor`, `TileImage`), datum formats, block-float decoding |
//! | [`format`] | Tensor tile formats (`TileFormat`): tile bytes, headers, exponent sections |
//!
//! `no_std`, no dependencies.
#![no_std]
#![forbid(unsafe_code)]

pub mod format;
pub mod tile;
