# tensix-isa

Hardware description of the Tenstorrent Blackhole chip for host and firmware:
instruction encoders, register and memory maps, NoC coordinate spaces, tile formats,
bit-exact numerics and the host/firmware protocol. `no_std`, no dependencies outside
the layer, `#![forbid(unsafe_code)]`; every crate builds for the host and for
`riscv32im-unknown-none-elf`.

`tensix-isa` re-exports the layer under one module tree (`tensix_isa::noc`,
`tensix_isa::dm`, …); each part is also its own crate.

| Crate | Modules | Contents |
|---|---|---|
| `tensix-insn` | `isa`, `adc`, `frontend` | Instruction encoders generated from the ISA specification, each with its provenance; address counters; MOP expander and replay |
| `tensix-regs` | `tensix`, `cfg` | Tile memory map, baby RISC-V cores, soft reset; the 820 configuration-register fields from `cfg_defines.h` |
| `tensix-noc` | `noc`, `dram`, `arc`, `eth` | NoC coordinates per NoC, NIU registers, atomics, multicast; GDDR6 channels; ARC telemetry; Ethernet tiles |
| `tensix-numerics` | `numerics` | `fma_bh` (port of the specification's `fma.c`), SFPU and stochastic rounding models |
| `tensix-tile` | `tile`, `format` | Tile layout in L1, datum formats, BFP8/4/2 sizes and decoding |
| `tensix-units` | `backend`, `sfpu`, `matrix`, `packer`, `unpacker`, `sync`, `scalar`, `hazard`, `tag_search` | Checked builders for the Tensix units; hazard checker |
| `tensix-fw-abi` | `l1`, `mailbox`, `dm`, `dataflow`, `ttinsn` | L1 region map, mailboxes, data-mover descriptors and records; host and firmware share one version |
| `tensix-layout` | – | Host tilization of strided tensors and back (`std`) |
| `tensix-isa` | all of the above | The layer in one crate |


## Generated tables

```bash
cargo xtask fetch-spec          # pinned tt-isa-documentation into vendor/
cargo xtask gen-isa [--check]   # crates/tensix-insn/src/isa/generated.rs
cargo xtask gen-cfg [--check]   # crates/tensix-regs/src/cfg/generated.rs
```

Pins are in `PINS.toml`. Measured Blackhole layouts cite the gate that measured them;
`gen-isa` looks the gates up in the `tests` directories of the `tensix-*` checkouts
next to this one, or in the directories named by `TENSIX_GATES`.

## License

Apache-2.0, see `LICENSE` and `NOTICE`.
