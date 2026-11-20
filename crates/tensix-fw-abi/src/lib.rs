//! The host/firmware protocol of the tensix stack on Tenstorrent Blackhole.
//!
//! | Module | Contents |
//! |---|---|
//! | [`l1`] | The fixed L1 region map and every mailbox block, checked at compile time |
//! | [`mailbox`] | Status words, role mailboxes, program region, guard deadlines |
//! | [`dm`] | The data-mover protocol: descriptors, list entries, element kinds, op records |
//! | [`dataflow`] | Dataflow programs between the movers and the Tensix threads |
//! | [`ttinsn`] | `.ttinsn` instruction blocks |
//!
//! Host and firmware must use the same version. `no_std`.
#![no_std]
#![forbid(unsafe_code)]

pub mod dataflow;
pub mod dm;
pub mod l1;
pub mod mailbox;
pub mod ttinsn;

use tensix_insn::isa;
use tensix_noc::{dram, noc};
use tensix_regs::tensix;
use tensix_tile::{format, tile};
