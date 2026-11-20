//! Bit-exact models of Tenstorrent Blackhole arithmetic.
//!
//! [`numerics`]: `fma_bh` (a port of the specification's `fma.c`), SFPU and stochastic
//! rounding models, and the source broadcast modes of the matrix unit (`SrcBroadcast`).
//!
//! `no_std`, no dependencies.
#![no_std]
#![forbid(unsafe_code)]

pub mod numerics;
