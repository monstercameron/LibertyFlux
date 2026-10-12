//! Differential test of the lifted head-swap against the verified rewrites
//! of batch 2.
//!
//! Each instance is one verified rewrite: it reads a head word, saves the
//! value into a second word, writes a static node into the head and returns
//! the value it read. The lifted method is `StaticLinks::replace_head`. For
//! each instance, CASES generated states are planted in every word of this
//! crate's table; the rewrite runs on the planted memory and the lifted
//! method runs on the same state, and the returned value and every word of
//! the table are compared. The states draw the head from the empty link, the
//! node itself, another node, and the save word from the empty link or a node.
//! Two deliberately wrong methods must be caught on every instance, and the
//! store-order method is shown to be equivalent (see its test).

#![cfg(target_arch = "x86")]

#[path = "support/instances_gen.rs"]
mod instances;

use std::sync::Mutex;

use instances::{INSTANCES, Instance, NODES, WORDS};
use lf_core::static_list::{Link, NodeId, StaticLinks, WordId};
use lf_linkdiff_b::{plant, read_back, relocated};

/// Generated states per instance.
const CASES: u32 = 1000;

/// One lock across the whole test program: the rewrites read and write
/// shared slots, so two tests must never run their cases at once.
static LOCK: Mutex<()> = Mutex::new(());

/// A deterministic generator (xorshift64), seeded per instance.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    /// A value in `0..n`; `n` is never zero here.
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The raw value of a link word: zero for the empty link, otherwise the
/// relocated address of a node of this batch.
fn raw_of_node(node: usize) -> u32 {
    relocated(NODES[node])
}

/// Translates a raw word of the rewrite's memory into a lifted link.
///
/// # Panics
///
/// When a word holds a value that is neither empty nor a node of this batch:
/// the generator never produces one, so that is a test bug.
fn link_of_raw(raw: u32) -> Link {
    if raw == 0 {
        return None;
    }
    let index = (0..NODES.len())
        .find(|&j| raw_of_node(j) == raw)
        .expect("test bug: a word holds a value that is no node of this batch");
    Some(NodeId(index))
}

/// Translates a lifted link back to the raw word the rewrite would hold.
fn raw_of_link(link: Link) -> u32 {
    match link {
        None => 0,
        Some(NodeId(j)) => raw_of_node(j),
    }
}

/// The position of an address in the instance's word table.
fn word_index(address: u32) -> usize {
    WORDS
        .iter()
        .position(|&w| w == address)
        .expect("test bug: an instance names a word outside the table")
}

/// The position of a node in the batch's node table.
fn node_index(address: u32) -> usize {
    NODES
        .iter()
        .position(|&n| n == address)
        .expect("test bug: an instance names a node outside the table")
}

/// A lifted method under test: the same contract as `replace_head`.
type Candidate = fn(&mut StaticLinks, WordId, WordId, NodeId) -> Link;

/// The lifted method itself.
fn lifted(links: &mut StaticLinks, head: WordId, save: WordId, node: NodeId) -> Link {
    links.replace_head(head, save, node)
}

/// Wrong version one: the head is left unchanged.
fn wrong_head_unchanged(
    links: &mut StaticLinks,
    head: WordId,
    save: WordId,
    _node: NodeId,
) -> Link {
    let old = links.word(head);
    links.set_word(save, old);
    old
}

/// Wrong version two: the save word takes the node instead of the old head.
fn wrong_save_takes_node(
    links: &mut StaticLinks,
    head: WordId,
    save: WordId,
    node: NodeId,
) -> Link {
    let old = links.word(head);
    links.set_word(save, Some(node));
    links.set_word(head, Some(node));
    old
}

/// Store-order version: the head is written before the save word. The two
/// words are distinct for every instance, so this must agree with `lifted`.
fn wrong_store_order(links: &mut StaticLinks, head: WordId, save: WordId, node: NodeId) -> Link {
    let old = links.word(head);
    links.set_word(head, Some(node));
    links.set_word(save, old);
    old
}

/// The first mismatch of a candidate against the instance's rewrite, if any.
fn first_mismatch(instance: &Instance, candidate: Candidate, seed: u64) -> Option<String> {
    let head_index = word_index(instance.head);
    let save_index = word_index(instance.save);
    let own = NodeId(node_index(instance.node));
    let mut rng = Rng(seed);
    for case in 0..CASES {
        let mut raw = vec![0u32; WORDS.len()];
        for word in raw.iter_mut() {
            *word = if rng.below(2) == 0 {
                0
            } else {
                raw_of_node(rng.below(NODES.len()))
            };
        }
        raw[head_index] = match rng.below(4) {
            0 => 0,
            1 => raw_of_node(own.0),
            2 => raw_of_node(rng.below(NODES.len())),
            _ => raw_of_node((own.0 + 1) % NODES.len()),
        };
        raw[save_index] = if rng.below(4) == 0 {
            0
        } else {
            raw_of_node(rng.below(NODES.len()))
        };
        for (word, &value) in WORDS.iter().zip(raw.iter()) {
            plant(*word, value);
        }
        let returned = (instance.call)();
        let after: Vec<u32> = WORDS.iter().map(|&w| read_back(w)).collect();

        let mut links = StaticLinks::new(WORDS.len());
        for (k, &value) in raw.iter().enumerate() {
            links.set_word(WordId(k), link_of_raw(value));
        }
        let old = candidate(&mut links, WordId(head_index), WordId(save_index), own);
        let got_return = raw_of_link(old);
        let got_words: Vec<u32> = (0..WORDS.len())
            .map(|k| raw_of_link(links.word(WordId(k))))
            .collect();
        if got_return != returned || got_words != after {
            return Some(format!(
                "{} case {}: rewrite returned {:#x}, lifted returned {:#x}; word {} differs or the words differ",
                instance.name,
                case,
                returned,
                got_return,
                (0..WORDS.len())
                    .find(|&k| got_words[k] != after[k])
                    .map_or_else(|| "none".to_string(), |k| format!("{k} at {:#x}", WORDS[k])),
            ));
        }
    }
    None
}

macro_rules! instance_test {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            let _guard = LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            let instance = &INSTANCES[$index];
            let seed = 0x9E37_79B9_7F4A_7C15 ^ ($index as u64 + 1);
            if let Some(message) = first_mismatch(instance, lifted, seed) {
                panic!("lifted method differs from the rewrite: {message}");
            }
            let teeth_head = first_mismatch(instance, wrong_head_unchanged, seed);
            assert!(
                teeth_head.is_some(),
                "wrong version (head unchanged) was not caught for {}",
                instance.name
            );
            let teeth_save = first_mismatch(instance, wrong_save_takes_node, seed);
            assert!(
                teeth_save.is_some(),
                "wrong version (save takes node) was not caught for {}",
                instance.name
            );
            let order = first_mismatch(instance, wrong_store_order, seed);
            assert!(
                order.is_none(),
                "store-order version differs for {}; the equivalence claim is wrong: {:?}",
                instance.name,
                order
            );
        }
    };
}

#[test]
fn every_instance_saves_into_a_word_other_than_its_head() {
    for instance in INSTANCES {
        assert_ne!(instance.head, instance.save, "{}", instance.name);
    }
}

instance_test!(t_fn_00e5e780, 0);
instance_test!(t_fn_00e5e7a0, 1);
instance_test!(t_fn_00e5e7c0, 2);
instance_test!(t_fn_00e5e7e0, 3);
instance_test!(t_fn_00e5e800, 4);
instance_test!(t_fn_00e5e820, 5);
instance_test!(t_fn_00e5e840, 6);
instance_test!(t_fn_00e5e860, 7);
instance_test!(t_fn_00e5e880, 8);
instance_test!(t_fn_00e5e8a0, 9);
instance_test!(t_fn_00e5e8c0, 10);
instance_test!(t_fn_00e5e8e0, 11);
instance_test!(t_fn_00e5ebe0, 12);
instance_test!(t_fn_00e5ec00, 13);
instance_test!(t_fn_00e5ec20, 14);
instance_test!(t_fn_00e5ec40, 15);
instance_test!(t_fn_00e5ec60, 16);
instance_test!(t_fn_00e5ec80, 17);
instance_test!(t_fn_00e5eca0, 18);
instance_test!(t_fn_00e5ecc0, 19);
instance_test!(t_fn_00e5ece0, 20);
instance_test!(t_fn_00e5ed00, 21);
instance_test!(t_fn_00e5ed20, 22);
instance_test!(t_fn_00e5ed40, 23);
instance_test!(t_fn_00e5ed60, 24);
instance_test!(t_fn_00e5ed80, 25);
instance_test!(t_fn_00e5eda0, 26);
instance_test!(t_fn_00e5edc0, 27);
instance_test!(t_fn_00e5ede0, 28);
instance_test!(t_fn_00e5ee00, 29);
instance_test!(t_fn_00e5ee20, 30);
instance_test!(t_fn_00e5ee40, 31);
instance_test!(t_fn_00e5ee60, 32);
instance_test!(t_fn_00e5ee80, 33);
instance_test!(t_fn_00e5eea0, 34);
instance_test!(t_fn_00e5eec0, 35);
instance_test!(t_fn_00e5eee0, 36);
instance_test!(t_fn_00e5ef00, 37);
instance_test!(t_fn_00e5ef20, 38);
instance_test!(t_fn_00e5ef40, 39);
instance_test!(t_fn_00e5ef60, 40);
instance_test!(t_fn_00e5ef80, 41);
instance_test!(t_fn_00e5f060, 42);
instance_test!(t_fn_00e5f080, 43);
instance_test!(t_fn_00e5f0a0, 44);
instance_test!(t_fn_00e5f0c0, 45);
instance_test!(t_fn_00e5f120, 46);
instance_test!(t_fn_00e5f140, 47);
instance_test!(t_fn_00e5f160, 48);
instance_test!(t_fn_00e5f180, 49);
instance_test!(t_fn_00e5f1a0, 50);
instance_test!(t_fn_00e5f1c0, 51);
instance_test!(t_fn_00e5f1e0, 52);
instance_test!(t_fn_00e5f200, 53);
instance_test!(t_fn_00e5f220, 54);
instance_test!(t_fn_00e5f240, 55);
instance_test!(t_fn_00e5f2a0, 56);
instance_test!(t_fn_00e5f2e0, 57);
