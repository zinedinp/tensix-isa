//! Generating the tables `tensix-insn` and `tensix-regs` are built from, fetching the
//! pinned specification inputs, and the checks that keep those pins honest.
//!
//! Dependency-free: it shells out to `curl`, `sha256sum` and `rustfmt`.

use std::process::ExitCode;

mod fetch;
mod gen_cfg;
mod gen_isa;
mod pin;
mod spec;
mod util;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let task = args.next();
    let result = match task.as_deref() {
        Some("gen-cfg") => gen_cfg::gen_cfg(args.any(|a| a == "--check")),
        Some("fetch-spec") => fetch::fetch_spec(args.any(|a| a == "--force")),
        Some("check-isa-sources") => gen_isa::check_sources(),
        Some("gen-isa") => gen_isa::generate(args.any(|a| a == "--check")),
        Some(other) => Err(format!("unknown task `{other}`\n\n{USAGE}")),
        None => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("xtask: {msg}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "\
usage: cargo xtask <task>

tasks:
  fetch-spec [--force]    download the pinned ISA specification tree into vendor/
                          and verify it against the pinned content digest
  gen-cfg [--check]       regenerate tensix-regs' backend-configuration field table
                          from the pinned cfg_defines.h (fetched on demand); --check
                          fails if the committed file is out of date
  gen-isa [--check]       regenerate tensix-insn's instruction table from the pinned
                          Bits32.lua, cross-checked against the specification's
                          syntax blocks; --check fails if the committed file is out
                          of date
  check-isa-sources       parse the pinned Bits32.lua and report what it holds";
