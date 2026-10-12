//! Host edge tests of the lifted constant-store method. These run on every
//! target and need no rewrite.

use lf_core::static_list::{StaticWords, WordId};

#[test]
fn a_pair_store_sets_both_words_and_leaves_the_rest() {
    let mut words = StaticWords::new(4);
    words.set_word(WordId(3), 0xFFFF_FFFF);
    words.store_pair(WordId(0), WordId(1), 3, 0);
    assert_eq!(words.word(WordId(0)), 3);
    assert_eq!(words.word(WordId(1)), 0);
    assert_eq!(words.word(WordId(2)), 0);
    assert_eq!(words.word(WordId(3)), 0xFFFF_FFFF);
}

#[test]
fn a_pair_store_on_one_word_keeps_the_second_value() {
    let mut words = StaticWords::new(1);
    words.store_pair(WordId(0), WordId(0), 1, 2);
    assert_eq!(words.word(WordId(0)), 2);
}

#[test]
#[should_panic]
fn an_id_outside_the_table_panics() {
    let mut words = StaticWords::new(1);
    words.store_pair(WordId(0), WordId(4), 0, 0);
}
