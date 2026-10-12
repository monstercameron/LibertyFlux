//! Host tests of the instance table. These run on every host and do not need the 32-bit rewrites.

use lf_24_filldiff::instances::INSTANCES;

#[test]
fn the_table_lists_6_instances() {
    assert_eq!(INSTANCES.len(), 6);
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = INSTANCES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), INSTANCES.len());
}
