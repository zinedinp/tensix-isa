//! `miraculix::formats::bfp::bfp_decode_rows_tensix` is the unpacker's decode: equal to
//! [`tile::bfp8_to_bf16`] and its BFP4/BFP2 forms for every shared exponent and every code.
use miraculix::formats::bfp::{BfpElem, bfp_decode_rows_tensix, bfp_decode_rows_tensix_scalar};
use tensix_tile::tile;

#[test]
fn miraculix_decodes_every_code_as_the_unpacker() {
    for (elem, bits, spec) in [
        (BfpElem::M7, 8u32, tile::bfp8_to_bf16 as fn(u8, u8) -> u16),
        (BfpElem::M3, 4, tile::bfp4_to_bf16),
        (BfpElem::M1, 2, tile::bfp2_to_bf16),
    ] {
        let per = 8 / bits as usize;
        for e in 0..=255u8 {
            for code in 0..(1u32 << bits) {
                let byte = (0..per).fold(0u8, |b, k| b | (code as u8) << (k as u32 * bits));
                let data = vec![byte; elem.row_bytes()];
                let want = spec(code as u8, e);
                for decode in [bfp_decode_rows_tensix, bfp_decode_rows_tensix_scalar] {
                    let mut out = [0u16; 16];
                    decode(elem, &[e], &data, &mut out);
                    assert!(
                        out.iter().all(|&o| o == want),
                        "{elem:?} exp {e} code {code:#x}: {out:x?} want {want:#06x}"
                    );
                }
            }
        }
    }
}
