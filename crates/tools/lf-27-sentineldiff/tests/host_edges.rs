//! Host tests of the lifted sentinel-table reset, on any host.
//!
//! These cover the edges a reader of the method would ask about: an empty run,
//! the bytes a record leaves alone, a flag before the run, the end index, and
//! the faults for a write outside the image, an index that overflows and a
//! flag outside the image. The differential test covers the four verified
//! rewrites on the 32-bit target.

use lf_core::sentinel_table::{EMPTY, Fault, SentinelTable};

#[test]
fn empty_run_writes_nothing_and_returns_its_start() {
    let mut image = vec![0xAB; 16];
    let table = SentinelTable {
        records: 0,
        stride: 8,
        flag: None,
    };
    assert_eq!(table.fill(&mut image, 4), Ok(4));
    assert!(image.iter().all(|&b| b == 0xAB));
}

#[test]
fn each_record_gets_the_marker_and_the_half_word_and_nothing_more() {
    let mut image = vec![0x55; 24];
    let table = SentinelTable {
        records: 2,
        stride: 8,
        flag: None,
    };
    assert_eq!(table.fill(&mut image, 0), Ok(16));
    // Record 0: marker at 0..4, zero half-word at 4..6, bytes 6..8 untouched.
    assert_eq!(&image[0..4], &EMPTY.to_le_bytes());
    assert_eq!(&image[4..6], &[0, 0]);
    assert_eq!(&image[6..8], &[0x55, 0x55]);
    // Record 1 is the same at 8..14; bytes 14..16 are not part of the run's writes.
    assert_eq!(&image[8..12], &EMPTY.to_le_bytes());
    assert_eq!(&image[12..14], &[0, 0]);
    assert_eq!(&image[14..16], &[0x55, 0x55]);
    // Bytes after the run are untouched.
    assert!(image[16..].iter().all(|&b| b == 0x55));
}

#[test]
fn flag_before_the_run_is_cleared_after_the_records() {
    let mut image = vec![0x77; 32];
    let table = SentinelTable {
        records: 1,
        stride: 8,
        flag: Some(-7),
    };
    assert_eq!(table.fill(&mut image, 8), Ok(16));
    assert_eq!(image[1], 0);
    assert_eq!(&image[8..12], &EMPTY.to_le_bytes());
    assert_eq!(image[0], 0x77);
}

#[test]
fn a_record_past_the_image_is_a_fault() {
    let mut image = vec![0; 12];
    let table = SentinelTable {
        records: 2,
        stride: 8,
        flag: None,
    };
    assert_eq!(table.fill(&mut image, 0), Err(Fault));
}

#[test]
fn a_flag_outside_the_image_is_a_fault() {
    let mut image = vec![0; 16];
    let table = SentinelTable {
        records: 0,
        stride: 8,
        flag: Some(-7),
    };
    assert_eq!(table.fill(&mut image, 2), Err(Fault));
}

#[test]
fn an_index_that_overflows_is_a_fault() {
    let mut image = vec![0; 16];
    let table = SentinelTable {
        records: u32::MAX,
        stride: u32::MAX,
        flag: None,
    };
    assert_eq!(table.fill(&mut image, 0), Err(Fault));
    let table = SentinelTable {
        records: 0,
        stride: 8,
        flag: None,
    };
    assert_eq!(table.fill(&mut image, usize::MAX), Ok(usize::MAX));
}
