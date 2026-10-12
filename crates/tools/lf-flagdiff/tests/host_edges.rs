//! Host tests of the lifted word test and its instance table. These run on every host and do not need the
//! 32-bit rewrites.

use lf_core::field_store::ObjectBytes;
use lf_core::static_list::{StaticWords, WordId};
use lf_core::word_test::instances::{GLOBAL_WORDS, PREDICATES};
use lf_core::word_test::{Compare, Predicate, Rhs, Source};

#[test]
fn the_table_lists_six_instances_over_two_global_words() {
    assert_eq!(PREDICATES.len(), 6);
    assert_eq!(GLOBAL_WORDS, 2);
}

#[test]
fn every_global_instance_names_a_word_inside_the_table() {
    for named in PREDICATES.iter() {
        if let Source::Global(WordId(number)) = named.predicate.source {
            assert!(number < GLOBAL_WORDS, "{}", named.name);
        }
    }
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = PREDICATES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), PREDICATES.len());
}

#[test]
fn a_parameter_right_hand_side_reads_the_parameter() {
    let object = ObjectBytes::from_bytes(&[0, 0, 0, 0, 0x2A, 0, 0, 0]);
    let words = StaticWords::new(GLOBAL_WORDS);
    let test = Predicate {
        source: Source::Field(4),
        compare: Compare::Equal,
        rhs: Rhs::Parameter,
    };
    assert_eq!(test.evaluate(&object, &words, 0x2A), 1);
    assert_eq!(test.evaluate(&object, &words, 0x2B), 0);
}
