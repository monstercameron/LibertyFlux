//! Host edge tests of the lifted head-swap. These run on every target and
//! need no rewrite: they state what a reader would ask about the method.

use lf_core::static_list::{NodeId, StaticLinks, WordId};

const HEAD: WordId = WordId(0);
const SAVE: WordId = WordId(1);

#[test]
fn an_empty_list_takes_the_node_and_saves_the_empty_link() {
    let mut links = StaticLinks::new(2);
    assert_eq!(links.replace_head(HEAD, SAVE, NodeId(7)), None);
    assert_eq!(links.word(HEAD), Some(NodeId(7)));
    assert_eq!(links.word(SAVE), None);
}

#[test]
fn a_chain_of_three_pushes_walks_back_in_reverse_order() {
    // Three nodes, each with its own link word, pushed in turn: walking the
    // list from the head follows the links back to the first node.
    let head = WordId(0);
    let links_of = [WordId(1), WordId(2), WordId(3)];
    let mut links = StaticLinks::new(4);
    for (n, &link) in links_of.iter().enumerate() {
        let old = links.replace_head(head, link, NodeId(n));
        if n == 0 {
            assert_eq!(old, None);
        } else {
            assert_eq!(old, Some(NodeId(n - 1)));
        }
    }
    let mut walked = Vec::new();
    let mut at = links.word(head);
    // Bounded by the node count: a cycle would stop the walk here.
    for _ in 0..links_of.len() {
        match at {
            Some(NodeId(n)) => {
                walked.push(n);
                at = links.word(links_of[n]);
            }
            None => break,
        }
    }
    assert_eq!(walked, vec![2, 1, 0]);
    assert_eq!(at, None);
}

#[test]
fn pushing_the_node_already_at_the_head_keeps_it_there() {
    let mut links = StaticLinks::new(2);
    links.set_word(HEAD, Some(NodeId(3)));
    assert_eq!(links.replace_head(HEAD, SAVE, NodeId(3)), Some(NodeId(3)));
    assert_eq!(links.word(HEAD), Some(NodeId(3)));
    assert_eq!(links.word(SAVE), Some(NodeId(3)));
}

#[test]
fn the_save_word_is_the_only_other_word_written() {
    let mut links = StaticLinks::new(3);
    links.set_word(WordId(2), Some(NodeId(9)));
    links.replace_head(HEAD, SAVE, NodeId(4));
    assert_eq!(links.word(WordId(2)), Some(NodeId(9)));
}

#[test]
#[should_panic(expected = "its own save word")]
fn a_head_cannot_save_into_itself() {
    let mut links = StaticLinks::new(1);
    links.replace_head(HEAD, HEAD, NodeId(0));
}

#[test]
#[should_panic]
fn an_id_outside_the_table_panics() {
    let mut links = StaticLinks::new(2);
    links.replace_head(HEAD, WordId(5), NodeId(0));
}
