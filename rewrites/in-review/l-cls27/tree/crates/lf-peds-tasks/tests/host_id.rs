//! Host tests of the task id lookup: the edge cases a reader would ask
//! about. The differential proof against the verified 32-bit rewrite lives
//! in the `lf-pedmoverdiff` test crate.

use lf_peds_tasks::ped_task::{DEFAULT_ID, NO_ID, TaskIdTable, TaskKey};

/// A table whose lookups are scripted and which logs what it was asked.
struct Scripted {
    primary: u32,
    fallback: u32,
    id_of_record: u32,
    asked_fallback: bool,
    keys_seen: Vec<u32>,
}

impl Scripted {
    fn new(primary: u32, fallback: u32, id_of_record: u32) -> Self {
        Scripted {
            primary,
            fallback,
            id_of_record,
            asked_fallback: false,
            keys_seen: Vec::new(),
        }
    }
}

impl TaskIdTable for Scripted {
    fn primary(&mut self, key: u32) -> u32 {
        self.keys_seen.push(key);
        self.primary
    }

    fn fallback(&mut self, key: u32) -> u32 {
        self.asked_fallback = true;
        self.keys_seen.push(key);
        self.fallback
    }

    fn record_id(&mut self, _record: u32) -> u32 {
        self.id_of_record
    }
}

#[test]
fn a_primary_record_answers_without_the_fallback() {
    let mut table = Scripted::new(0x100, 0x200, 42);
    let id = TaskKey { key: 7 }.id_or_default(&mut table);
    assert_eq!(id, 42);
    assert!(!table.asked_fallback);
    assert_eq!(table.keys_seen, vec![7]);
}

#[test]
fn a_missing_primary_record_falls_back_with_the_same_key() {
    let mut table = Scripted::new(0, 0x200, 42);
    let id = TaskKey { key: 9 }.id_or_default(&mut table);
    assert_eq!(id, 42);
    assert!(table.asked_fallback);
    assert_eq!(table.keys_seen, vec![9, 9]);
}

#[test]
fn the_sentinel_id_gives_the_default() {
    let mut table = Scripted::new(0x100, 0x200, NO_ID);
    assert_eq!(TaskKey { key: 1 }.id_or_default(&mut table), DEFAULT_ID);
    assert_eq!(DEFAULT_ID, 0x31);
}

#[test]
fn an_id_next_to_the_sentinel_is_kept() {
    let mut table = Scripted::new(0x100, 0x200, NO_ID - 1);
    assert_eq!(TaskKey { key: 1 }.id_or_default(&mut table), NO_ID - 1);
}

#[test]
fn a_zero_id_is_an_id_not_a_miss() {
    // Only the record lookup's zero answer is a miss; a zero id word is a
    // real id and is returned as it is.
    let mut table = Scripted::new(0x100, 0x200, 0);
    assert_eq!(TaskKey { key: 1 }.id_or_default(&mut table), 0);
    assert!(!table.asked_fallback);
}
