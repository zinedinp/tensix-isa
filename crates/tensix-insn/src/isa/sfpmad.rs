//! The cases `SFPMAD.md` lists where the hardware's automatic stall after an `SFPMAD`
//! misses a dependency.

use super::Instruction;

/// Modifier bits named by `SFPMAD.md`'s list of cases automatic stalling misses.
///
/// Kept here rather than generated: they live in prose tables on each
/// instruction's page, not in the encoding diagrams, and the generated layer
/// deliberately describes bits rather than meanings.
pub mod stall_modes {
    /// `SFPAND_MOD1_USE_VB` / `SFPOR_MOD1_USE_VB`, new in Blackhole.
    pub const USE_VB: u32 = 1;
    /// `SFPSWAP_MOD1_SWAP` — the one `SFPSWAP` mode stalling handles correctly.
    pub const SWAP_UNCONDITIONAL: u32 = 0;
    /// `SFPSHFT2` modes the stalling logic does not see reads in
    /// (`SUBVEC_SHFLROR1_AND_COPY4`, `SUBVEC_SHFLROR1`, `SUBVEC_SHFLSHR1`), and the
    /// two where it watches the wrong register (`SHFT_LREG`, `SHFT_IMM`).
    pub const SHFT2_MISSED: [u32; 5] = [2, 3, 4, 5, 6];
}

impl Instruction {
    /// Does an `SFPNOP` have to be inserted between an `SFPMAD` and this
    /// instruction?
    ///
    /// On Blackhole the two-cycle `SFPMAD` latency is handled by hardware: it
    /// stalls a thread that presents an instruction reading what the `SFPMAD`
    /// wrote (`SFPMAD.md`, "Instruction scheduling"). That is a change from
    /// Wormhole, and it means the blanket "always follow a multiply with a NOP"
    /// rule is wrong here — it would just cost a cycle.
    ///
    /// What survives is a specific list of cases the stalling logic fails to
    /// detect, which `SFPMAD.md` enumerates as hardware bugs. Three are
    /// unconditional; four depend on the consuming instruction's `Mod1`, and since
    /// an [`Instruction`] carries its definition, that can be read rather than
    /// assumed.
    ///
    /// Separately, automatic stalling does not apply at all inside an
    /// `SFPLOADMACRO` sequence — which ttsim does not implement, so that case
    /// cannot arise against the simulator.
    pub fn stalls_automatically_after_mad(self) -> bool {
        let mod1 = self.operand("Mod1").unwrap_or(0);
        let missed = match self.def().mnemonic() {
            // The stalling logic ignores `USE_VB`, so it believes these always read
            // `VD` and never `VB`.
            "SFPAND" | "SFPOR" => mod1 & stall_modes::USE_VB != 0,
            // It does not realise these read from `VD` at all, nor that `SFPCONFIG`
            // can read `LReg[0]`.
            "SFPIADD" | "SFPSHFT" | "SFPCONFIG" => true,
            // Every mode but the unconditional swap compares `VC` and `VD` on its
            // first cycle, and the stalling logic sees no reads there.
            "SFPSWAP" => mod1 != stall_modes::SWAP_UNCONDITIONAL,
            // Three modes it sees no reads in at all, and two where it watches `VD`
            // while the instruction reads `VB`.
            "SFPSHFT2" => {
                let mut i = 0;
                let mut found = false;
                while i < stall_modes::SHFT2_MISSED.len() {
                    found |= stall_modes::SHFT2_MISSED[i] == mod1;
                    i += 1;
                }
                found
            }
            _ => false,
        };
        !missed
    }
}
