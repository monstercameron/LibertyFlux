//! Host tests of the lifted word copy and its instance table. These run on every host and do not need the
//! 32-bit rewrites.

use lf_core::static_list::{StaticWords, WordId};
use lf_core::word_copy::instances::{COPIES, WORD_COUNT};

#[test]
fn the_table_lists_eleven_instances_over_fifteen_words() {
    assert_eq!(COPIES.len(), 11);
    assert_eq!(WORD_COUNT, 15);
}

#[test]
fn every_instance_names_words_inside_the_table() {
    for named in COPIES.iter() {
        assert!(named.copy.source.0 < WORD_COUNT, "{}", named.name);
        assert!(named.copy.destination.0 < WORD_COUNT, "{}", named.name);
        assert_ne!(named.copy.source, named.copy.destination, "{}", named.name);
    }
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = COPIES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), COPIES.len());
}

#[test]
fn every_instance_applies_to_a_table_of_the_full_size() {
    for named in COPIES.iter() {
        let mut words = StaticWords::new(WORD_COUNT);
        words.set_word(named.copy.source, 0x1357_9BDF);
        let returned = named.copy.apply(&mut words);
        assert_eq!(returned, 0, "{}", named.name);
        assert_eq!(words.word(named.copy.destination), 0x1357_9BDF, "{}", named.name);
    }
}
