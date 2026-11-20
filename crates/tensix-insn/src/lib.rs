//! Tensix instruction encoders for Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`isa`] | The Tensix instruction set; `isa::generated` comes from the ISA specification (`cargo xtask gen-isa`), each encoding with its `Provenance` |
//! | [`adc`] | Address counters (ADC) of the unpackers, math and packer threads |
//! | [`frontend`] | The frontend's MOP expander and replay buffer |
//!
//! `no_std`, no dependencies: built for the host and for `riscv32im` firmware.
#![no_std]
#![forbid(unsafe_code)]

pub mod adc;
pub mod frontend;
pub mod isa;
