//! Host tests of the task owner replacement on the 64-bit host.

use lf_peds_tasks::ped_task::{OwnerCallees, TaskOwner};

#[derive(Default)]
struct Calls {
    log: Vec<String>,
}

impl OwnerCallees for Calls {
    fn release(&mut self, previous: u32) {
        self.log.push(format!("release {previous}"));
    }

    fn deallocate(&mut self, previous: u32) {
        self.log.push(format!("deallocate {previous}"));
    }
}

#[test]
fn a_null_previous_owner_only_stores() {
    let mut owner = TaskOwner::default();
    let mut calls = Calls::default();
    owner.replace_owner(9, &mut calls);
    assert_eq!(owner.object, 9);
    assert!(calls.log.is_empty());
}

#[test]
fn an_unchanged_owner_makes_no_calls() {
    let mut owner = TaskOwner { object: 4 };
    let mut calls = Calls::default();
    owner.replace_owner(4, &mut calls);
    assert_eq!(owner.object, 4);
    assert!(calls.log.is_empty());
}

#[test]
fn a_changed_owner_is_released_then_deallocated_before_the_store() {
    let mut owner = TaskOwner { object: 4 };
    let mut calls = Calls::default();
    owner.replace_owner(0, &mut calls);
    assert_eq!(owner.object, 0);
    assert_eq!(calls.log, ["release 4", "deallocate 4"]);
}
