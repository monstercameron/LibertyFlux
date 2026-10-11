//! Host tests of the task manager's slot arrays on the 64-bit host.

use lf_peds_tasks::ped_task::{
    FIRST_BOMB_KIND, ManagerCallees, PRIMARY_COUNT, SECOND_BOMB_KIND, TaskSlots,
};

#[derive(Default)]
struct Interface {
    calls: Vec<String>,
    final_answer: u32,
}

impl ManagerCallees for Interface {
    fn request_task(&mut self, kind: u32) {
        self.calls.push(format!("request {kind}"));
    }

    fn prepare_tasks(&mut self) {
        self.calls.push("prepare".to_owned());
    }

    fn refresh_tasks(&mut self) {
        self.calls.push("refresh".to_owned());
    }

    fn final_update(&mut self) -> u32 {
        self.calls.push("final".to_owned());
        self.final_answer
    }
}

#[test]
fn first_occupied_is_the_count_when_all_are_null() {
    assert_eq!(TaskSlots::default().first_occupied(), PRIMARY_COUNT);
}

#[test]
fn first_occupied_finds_the_lowest_index() {
    let mut slots = TaskSlots::default();
    slots.primary[3] = 7;
    slots.primary[1] = 9;
    assert_eq!(slots.first_occupied(), 1);
}

#[test]
fn contains_reads_only_the_secondary_array() {
    let mut slots = TaskSlots::default();
    slots.primary[0] = 5;
    assert!(!slots.contains(5));
    slots.secondary[5] = 5;
    assert!(slots.contains(5));
    // Empty secondary entries are zero, so zero is found there.
    assert!(slots.contains(0));
}

#[test]
fn bomb_request_prefers_the_first_slot() {
    let mut slots = TaskSlots::default();
    slots.primary[2] = 1;
    let mut interface = Interface::default();
    slots.request_bomb_task(&mut interface);
    assert_eq!(interface.calls, [format!("request {SECOND_BOMB_KIND}")]);

    slots.primary[1] = 1;
    let mut interface = Interface::default();
    slots.request_bomb_task(&mut interface);
    assert_eq!(interface.calls, [format!("request {FIRST_BOMB_KIND}")]);
}

#[test]
fn bomb_request_is_silent_when_both_slots_are_null() {
    let mut interface = Interface::default();
    TaskSlots::default().request_bomb_task(&mut interface);
    TaskSlots::default().request_first_bomb_task(&mut interface);
    assert!(interface.calls.is_empty());
}

#[test]
fn update_sequence_returns_the_final_answer_after_two_preparations() {
    let mut interface = Interface {
        final_answer: 42,
        ..Interface::default()
    };
    let answer = TaskSlots::default().update_sequence(&mut interface);
    assert_eq!(answer, 42);
    assert_eq!(interface.calls, ["prepare", "refresh", "final"]);
}
