//! Host tests of the task event table: the edge cases a reader would ask
//! about, on the 64-bit host. The 32-bit differential cases live in the
//! `lf-eventdiff` test crate.

use lf_peds_tasks::ped_task::{
    DELAY_GAP, EventCallees, EventObjects, EventRecord, EventTable, QUEUED_KIND, RECORD_COUNT,
    REDUCED_THRESHOLD, STANDARD_THRESHOLD,
};

/// A helper stand-in that answers from fixed values and logs each call.
#[derive(Default)]
struct Fake {
    delay: u32,
    mode: u32,
    calls: Vec<String>,
}

impl EventCallees for Fake {
    fn delay_ticks(&mut self, task_id: u32) -> u32 {
        self.calls.push(format!("delay {task_id}"));
        self.delay
    }

    fn selection_mode(&mut self) -> u32 {
        self.calls.push("mode".to_owned());
        self.mode
    }

    fn release_record(&mut self, record: usize) {
        self.calls.push(format!("release {record}"));
    }

    fn proximity_notify(&mut self, record: usize) {
        self.calls.push(format!("proximity {record}"));
    }

    fn release_object(&mut self, object: u32, record: usize) {
        self.calls
            .push(format!("release object {object} at {record}"));
    }

    fn retain_object(&mut self, object: u32, record: usize) {
        self.calls
            .push(format!("retain object {object} at {record}"));
    }
}

fn queued(event: u32, object: u32, deadline: u32) -> EventRecord {
    EventRecord {
        event,
        kind: QUEUED_KIND,
        object,
        deadline,
        flag: 0,
    }
}

fn table_with(entries: &[(usize, EventRecord)]) -> EventTable {
    let mut table = EventTable::default();
    for &(index, record) in entries {
        table.records[index] = record;
    }
    table
}

#[test]
fn refresh_leaves_a_deadline_exactly_at_the_gap() {
    let now = 5_000;
    let mut table = table_with(&[(0, queued(1, 7, now + DELAY_GAP))]);
    let mut fake = Fake {
        delay: 40,
        ..Fake::default()
    };
    table.refresh_matching_deadlines(7, now, &mut fake);
    assert_eq!(table.records[0].deadline, now + DELAY_GAP);
    assert!(fake.calls.is_empty());
}

#[test]
fn refresh_moves_a_deadline_one_past_the_gap() {
    let now = 5_000;
    let mut table = table_with(&[(0, queued(1, 7, now + DELAY_GAP + 1))]);
    let mut fake = Fake {
        delay: 40,
        ..Fake::default()
    };
    table.refresh_matching_deadlines(7, now, &mut fake);
    assert_eq!(table.records[0].deadline, now + 40);
    assert_eq!(fake.calls, ["delay 7"]);
}

#[test]
fn refresh_compares_unsigned_so_a_deadline_before_now_is_far_beyond() {
    // A deadline before the current time is a huge unsigned difference, so
    // it is refreshed, as the original's unsigned compare does.
    let now = 5_000;
    let mut table = table_with(&[(3, queued(1, 9, now - 1))]);
    let mut fake = Fake {
        delay: 0,
        ..Fake::default()
    };
    table.refresh_matching_deadlines(9, now, &mut fake);
    assert_eq!(table.records[3].deadline, now);
}

#[test]
fn refresh_wraps_the_new_deadline() {
    let now = u32::MAX - 10;
    let mut table = table_with(&[(0, queued(1, 2, now.wrapping_add(DELAY_GAP + 1000)))]);
    let mut fake = Fake {
        delay: 100,
        ..Fake::default()
    };
    table.refresh_matching_deadlines(2, now, &mut fake);
    assert_eq!(table.records[0].deadline, now.wrapping_add(100));
}

#[test]
fn selection_keeps_the_first_record_on_a_tie() {
    let mut table = table_with(&[(4, queued(1, 3, 50)), (9, queued(1, 3, 50))]);
    let mut fake = Fake {
        mode: 0,
        ..Fake::default()
    };
    table.release_lowest_deadline_record(3, &mut fake);
    // Two matches, below the standard threshold: nothing is released.
    assert!(fake.calls.iter().all(|call| call == "mode"));

    let mut many = EventTable::default();
    for index in 0..STANDARD_THRESHOLD as usize {
        many.records[index] = queued(1, 3, 50);
    }
    many.records[2].deadline = 50;
    many.records[7] = queued(1, 3, 10);
    many.records[5] = queued(1, 3, 10);
    let mut fake = Fake {
        mode: 0,
        ..Fake::default()
    };
    many.release_lowest_deadline_record(3, &mut fake);
    assert_eq!(fake.calls, ["mode", "release 5"]);
}

#[test]
fn selection_threshold_follows_the_mode_low_byte() {
    let entries: Vec<(usize, EventRecord)> = (0..REDUCED_THRESHOLD as usize)
        .map(|index| (index, queued(1, 4, 100 + index as u32)))
        .collect();

    let mut table = table_with(&entries);
    let mut reduced = Fake {
        mode: 0x101,
        ..Fake::default()
    };
    table.release_lowest_deadline_record(4, &mut reduced);
    assert_eq!(reduced.calls, ["mode", "release 0"]);

    let mut table = table_with(&entries);
    let mut standard = Fake {
        mode: 0x200,
        ..Fake::default()
    };
    table.release_lowest_deadline_record(4, &mut standard);
    assert_eq!(standard.calls, ["mode"]);
}

#[test]
fn lookup_treats_zero_as_no_key_and_finds_the_first_match() {
    let table = table_with(&[(2, queued(9, 0, 0)), (6, queued(9, 0, 0))]);
    assert_eq!(table.find_by_event(0), None);
    assert_eq!(table.find_by_event(9), Some(2));
    assert_eq!(table.find_by_event(8), None);
}

#[test]
fn object_match_rejects_indices_past_the_table() {
    let table = table_with(&[(RECORD_COUNT - 1, queued(1, 0, 0))]);
    assert!(table.holds_object(RECORD_COUNT as u32 - 1, 0));
    assert!(!table.holds_object(RECORD_COUNT as u32, 0));
    assert!(!table.holds_object(u32::MAX, 0));
}

#[test]
fn flag_stores_only_the_low_byte_and_skips_inactive_records() {
    let mut table = table_with(&[
        (
            0,
            EventRecord {
                event: 1,
                kind: 5,
                object: 0,
                deadline: 0,
                flag: 0,
            },
        ),
        (
            1,
            EventRecord {
                event: 0,
                kind: 5,
                object: 0,
                deadline: 0,
                flag: 0,
            },
        ),
    ]);
    table.set_event_flag(5, 0x1FF);
    assert_eq!(table.records[0].flag, 0xFF);
    assert_eq!(table.records[1].flag, 0);
    table.set_event_flag(5, 0x100);
    assert_eq!(table.records[0].flag, 0);
}

#[test]
fn release_passes_record_indices_in_table_order() {
    let table = table_with(&[
        (1, queued(1, 0, 0)),
        (
            3,
            EventRecord {
                event: 1,
                kind: 0,
                object: 0,
                deadline: 0,
                flag: 0,
            },
        ),
        (30, queued(0, 0, 0)),
    ]);
    let mut fake = Fake::default();
    table.release_active(&mut fake);
    assert_eq!(fake.calls, ["release 1", "release 3"]);

    let mut fake = Fake::default();
    table.release_nonqueued(&mut fake);
    assert_eq!(fake.calls, ["release 3"]);
}

#[test]
fn replace_ignores_an_index_past_the_table() {
    let mut table = table_with(&[(0, queued(1, 5, 0))]);
    let before = table.clone();
    let mut fake = Fake::default();
    table.replace_object(RECORD_COUNT as u32, 6, &mut fake);
    table.replace_object(u32::MAX, 6, &mut fake);
    assert_eq!(table, before);
    assert!(fake.calls.is_empty());
}

#[test]
fn replace_releases_then_stores_then_retains() {
    let mut table = table_with(&[(4, queued(1, 5, 0))]);
    let mut fake = Fake::default();
    table.replace_object(4, 6, &mut fake);
    assert_eq!(table.records[4].object, 6);
    assert_eq!(
        fake.calls,
        ["release object 5 at 4", "retain object 6 at 4"]
    );
}

#[test]
fn replace_with_null_releases_only_and_null_current_retains_only() {
    let mut table = table_with(&[(0, queued(1, 5, 0)), (1, queued(1, 0, 0))]);
    let mut fake = Fake::default();
    table.replace_object(0, 0, &mut fake);
    assert_eq!(table.records[0].object, 0);
    assert_eq!(fake.calls, ["release object 5 at 0"]);

    let mut fake = Fake::default();
    table.replace_object(1, 8, &mut fake);
    assert_eq!(table.records[1].object, 8);
    assert_eq!(fake.calls, ["retain object 8 at 1"]);
}

#[test]
fn replace_with_the_same_object_does_nothing() {
    let mut table = table_with(&[(2, queued(1, 5, 0))]);
    let mut fake = Fake::default();
    table.replace_object(2, 5, &mut fake);
    assert!(fake.calls.is_empty());
    assert_eq!(table.records[2].object, 5);
}

/// Fixed answers for the nearby check: each event object is a handle with
/// a type byte, a vector id and a position.
#[derive(Default)]
struct Objects {
    /// (handle, type byte, vector id, position) per event.
    events: Vec<(u32, u8, u32, [f32; 3])>,
}

impl EventObjects for Objects {
    fn type_byte(&self, event: u32) -> u8 {
        self.events.iter().find(|e| e.0 == event).map_or(0, |e| e.1)
    }

    fn vector_id(&self, event: u32) -> u32 {
        self.events.iter().find(|e| e.0 == event).map_or(0, |e| e.2)
    }

    fn position(&self, event: u32) -> [f32; 3] {
        self.events
            .iter()
            .find(|e| e.0 == event)
            .map_or([0.0; 3], |e| e.3)
    }
}

fn nearby_table(event: u32) -> EventTable {
    let mut table = EventTable::default();
    table.records[0].event = event;
    table
}

#[test]
fn nearby_is_strict_at_the_threshold() {
    // A distance of exactly half a unit squares to the threshold: not nearby.
    let objects = Objects {
        events: vec![(1, 0x02, 9, [0.5, 0.0, 0.0])],
    };
    let mut fake = Fake::default();
    nearby_table(1).check_nearby_event(9, [0.0, 0.0, 0.0], &objects, &mut fake);
    assert!(fake.calls.is_empty());

    let objects = Objects {
        events: vec![(1, 0x02, 9, [0.49, 0.0, 0.0])],
    };
    nearby_table(1).check_nearby_event(9, [0.0, 0.0, 0.0], &objects, &mut fake);
    assert_eq!(fake.calls, ["proximity 0"]);
}

#[test]
fn nearby_treats_low_types_as_vector_zero() {
    let objects = Objects {
        events: vec![(1, 0x01, 77, [0.0; 3])],
    };
    let mut fake = Fake::default();
    // A type below two is attached to vector zero, whatever its stored word.
    nearby_table(1).check_nearby_event(0, [0.0; 3], &objects, &mut fake);
    assert_eq!(fake.calls, ["proximity 0"]);

    let mut fake = Fake::default();
    nearby_table(1).check_nearby_event(77, [0.0; 3], &objects, &mut fake);
    assert!(fake.calls.is_empty());
}

#[test]
fn nearby_skips_a_nan_distance_and_a_null_event() {
    let objects = Objects {
        events: vec![(1, 0x02, 9, [f32::NAN, 0.0, 0.0])],
    };
    let mut fake = Fake::default();
    nearby_table(1).check_nearby_event(9, [0.0; 3], &objects, &mut fake);
    assert!(fake.calls.is_empty());

    let objects = Objects::default();
    nearby_table(0).check_nearby_event(9, [0.0; 3], &objects, &mut fake);
    assert!(fake.calls.is_empty());
}
