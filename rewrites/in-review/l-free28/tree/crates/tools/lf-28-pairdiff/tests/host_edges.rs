//! Host tests of the instance table. These run on every host and do not need the 32-bit rewrites.

use lf_28_pairdiff::instances::INSTANCES;

#[test]
fn the_table_lists_3_instances() {
    assert_eq!(INSTANCES.len(), 3);
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = INSTANCES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), INSTANCES.len());
}
