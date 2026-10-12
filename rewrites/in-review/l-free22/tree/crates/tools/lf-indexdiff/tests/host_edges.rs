//! Host tests of the indexed-read instance table. These run on every host and need no 32-bit rewrites.

use lf_core::indexed_read::instances::READS;

#[test]
fn the_table_lists_six_instances() {
    assert_eq!(READS.len(), 6);
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = READS.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), READS.len());
}

#[test]
fn every_instance_reads_inside_a_table_of_the_full_size() {
    for named in READS.iter() {
        let last = named.read.offset(255) as usize;
        let width = match named.read.width {
            lf_core::indexed_read::EntryWidth::Byte => 1,
            lf_core::indexed_read::EntryWidth::Word => 4,
        };
        assert!(last + width <= 0x2000, "{}", named.name);
    }
}

