//! Facts that tie two crates of the layer together.
use tensix_isa::{dm, format::TileFormat, mailbox, noc::probe};

/// The stack's tile slots are the formats' slots.
#[test]
fn the_movers_tile_slots_are_the_formats_slots() {
    assert_eq!(TileFormat::FP32.slot_bytes(), dm::TILE_SLOT);
    assert_eq!(TileFormat::BF16.slot_bytes(), dm::BF16_TILE_SLOT);
}

/// The NoC probe's regions sit clear of the firmware image and the mailboxes.
#[test]
fn the_probe_regions_are_clear_of_the_image_and_mailbox() {
    use probe::*;
    for (start, end) in [
        (SCRIPT, REQUESTS + MAX_REQUESTS as u64 * REQUEST_STRIDE),
        (RESULTS, RESULTS + MAX_REQUESTS as u64 * RESULT_STRIDE),
        (STAGE, STAGE + 4),
        (SNAPSHOTS, SNAPSHOTS + MAX_REQUESTS as u64 * SNAPSHOT_STRIDE),
        (DATA, DATA + 0x1000),
    ] {
        assert!(start >= dm::IMAGE_BASE + dm::IMAGE_MAX, "{start:#x}");
        assert!(end <= mailbox::MAILBOX_BASE, "{end:#x}");
    }
}
