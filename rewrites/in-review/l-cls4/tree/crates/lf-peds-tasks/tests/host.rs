//! Host edge tests for the lifted event handler.
//!
//! One case per question a reader of the code would ask: the clearing
//! kinds and their neighbours, the payload gate, the dispatch-or-clear
//! split, the one-to-three poll shapes, and the adopt path. Bit-exactness
//! against the verified rewrites is proven by the 32-bit differential
//! crate, not here.

use lf_core::Handle32;
use lf_peds_tasks::event_handler::registry::{self, State};
use lf_peds_tasks::event_handler::{
    KIND_CLEAR, KIND_RESET_B, KIND_TYPE_CLEAR_B, EventDispatch, EventHandler, EventPayload,
    EventSource, Owner, Task,
};

fn owner(v: u32) -> Option<Handle32<Owner>> {
    Handle32::new(v)
}

fn task(v: u32) -> Option<Handle32<Task>> {
    Handle32::new(v)
}

fn payload(v: u32) -> Option<Handle32<EventPayload>> {
    Handle32::new(v)
}

#[test]
fn registry_counts_five_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 5);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 46);
    assert_eq!(registry::ROWS.len(), 51);
    for row in registry::ROWS {
        assert_eq!(row.class, "EventHandler");
        if row.state == State::Proven {
            assert!(
                ["vf29", "vf31", "vf34", "vf36", "vf37"].contains(&row.method),
                "unexpected proven row {}",
                row.method
            );
        }
    }
}

#[test]
fn clearing_kinds_have_their_values() {
    assert_eq!(KIND_CLEAR, 0xC8);
    assert_eq!(KIND_TYPE_CLEAR_B, 0x201);
    assert_eq!(KIND_RESET_B, 0x3A7);
}

#[test]
fn handler_round_trips_its_words() {
    let h = EventHandler::new(owner(0x1234), task(0x5678));
    assert_eq!(h.owner(), owner(0x1234));
    assert_eq!(h.pending(), task(0x5678));
    let n = EventHandler::new(None, None);
    assert_eq!(n.owner(), None);
    assert_eq!(n.pending(), None);
}

// The type-clear slot.

#[test]
fn type_clear_drops_task_on_either_kind() {
    for kind in [KIND_CLEAR, KIND_TYPE_CLEAR_B] {
        let mut h = EventHandler::new(owner(1), task(0xABCD));
        assert_eq!(h.clear_pending_on_type(kind), kind);
        assert_eq!(h.pending(), None);
    }
}

#[test]
fn type_clear_keeps_task_next_to_the_kinds() {
    for kind in [0, 1, 0xC7, 0xC9, 0x200, 0x202, 0x3A7, u32::MAX] {
        let mut h = EventHandler::new(owner(1), task(0xABCD));
        assert_eq!(h.clear_pending_on_type(kind), kind);
        assert_eq!(h.pending(), task(0xABCD), "kind {kind:#x}");
    }
}

#[test]
fn type_clear_on_empty_stays_empty() {
    let mut h = EventHandler::new(None, None);
    assert_eq!(h.clear_pending_on_type(KIND_CLEAR), KIND_CLEAR);
    assert_eq!(h.pending(), None);
}

// The payload-gated reset slot.

#[test]
fn reset_needs_a_payload_and_a_kind() {
    // No payload: nothing happens even for a reset kind.
    let mut h = EventHandler::new(owner(1), task(9));
    h.reset_pending_on_kind(KIND_CLEAR, false);
    assert_eq!(h.pending(), task(9));
    // Payload but another kind: nothing happens.
    for kind in [0, 0xC7, 0xC9, 0x201, 0x3A6, 0x3A8, u32::MAX] {
        let mut h = EventHandler::new(owner(1), task(9));
        h.reset_pending_on_kind(kind, true);
        assert_eq!(h.pending(), task(9), "kind {kind:#x}");
    }
    // Both: cleared.
    for kind in [KIND_CLEAR, KIND_RESET_B] {
        let mut h = EventHandler::new(owner(1), task(9));
        h.reset_pending_on_kind(kind, true);
        assert_eq!(h.pending(), None);
    }
}

// The forward-or-clear slot.

struct Recorder {
    calls: Vec<(u32, Option<Handle32<EventPayload>>)>,
}

impl EventDispatch for Recorder {
    fn dispatch_event(&mut self, kind: u32, payload: Option<Handle32<EventPayload>>) {
        self.calls.push((kind, payload));
    }
}

#[test]
fn forward_clears_on_the_clear_kind_only() {
    let mut h = EventHandler::new(owner(1), task(7));
    let mut rec = Recorder { calls: Vec::new() };
    h.forward_or_clear(KIND_CLEAR, payload(0xFF), &mut rec);
    assert_eq!(h.pending(), None);
    assert!(rec.calls.is_empty());

    for (kind, pl) in [(0, None), (0xC7, payload(1)), (0xC9, None), (0x201, payload(2))] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut rec = Recorder { calls: Vec::new() };
        h.forward_or_clear(kind, pl, &mut rec);
        assert_eq!(h.pending(), task(7), "kind {kind:#x}");
        assert_eq!(rec.calls, vec![(kind, pl)], "kind {kind:#x}");
    }
}

// The settle slot.

/// Scripted event: answers polls from a list, clones one fixed task.
struct Scripted {
    polls: Vec<Option<Handle32<Owner>>>,
    clone_answer: Option<Handle32<Task>>,
    poll_calls: usize,
    clone_calls: usize,
}

impl EventSource for Scripted {
    fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
        let i = self.poll_calls.min(self.polls.len().saturating_sub(1));
        self.poll_calls += 1;
        self.polls[i]
    }

    fn clone_task(&mut self) -> Option<Handle32<Task>> {
        self.clone_calls += 1;
        self.clone_answer
    }
}

#[test]
fn settle_stops_on_first_null() {
    let h = EventHandler::new(owner(0x1111), task(5));
    let mut ev = Scripted {
        polls: vec![None],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    h.settle_poll(&mut ev);
    assert_eq!(ev.poll_calls, 1);
    assert_eq!(h.pending(), task(5));
}

#[test]
fn settle_stops_when_second_matches_owner() {
    let h = EventHandler::new(owner(0x1111), task(5));
    let mut ev = Scripted {
        polls: vec![owner(0x2222), owner(0x1111)],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    h.settle_poll(&mut ev);
    assert_eq!(ev.poll_calls, 2);
}

#[test]
fn settle_polls_thrice_otherwise() {
    let h = EventHandler::new(owner(0x1111), task(5));
    // Second answer differs: a third call follows, whatever it says.
    for third in [None, owner(0x1111), owner(0x3333)] {
        let mut ev = Scripted {
            polls: vec![owner(0x2222), owner(0x3333), third],
            clone_answer: None,
            poll_calls: 0,
            clone_calls: 0,
        };
        h.settle_poll(&mut ev);
        assert_eq!(ev.poll_calls, 3);
    }
}

#[test]
fn settle_compares_against_a_null_owner() {
    // A null second answer matches a null owner: two calls.
    let h = EventHandler::new(None, task(5));
    let mut ev = Scripted {
        polls: vec![owner(0x2222), None],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    h.settle_poll(&mut ev);
    assert_eq!(ev.poll_calls, 2);
    // A live second answer differs from a null owner: three calls.
    let mut ev = Scripted {
        polls: vec![owner(0x2222), owner(0x3333)],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    h.settle_poll(&mut ev);
    assert_eq!(ev.poll_calls, 3);
}

// The adopt slot.

#[test]
fn adopt_stores_and_answers_the_clone() {
    for answer in [None, task(1), task(u32::MAX)] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut ev = Scripted {
            polls: Vec::new(),
            clone_answer: answer,
            poll_calls: 0,
            clone_calls: 0,
        };
        assert_eq!(h.adopt_cloned_task(&mut ev), answer);
        assert_eq!(h.pending(), answer);
        assert_eq!(ev.clone_calls, 1);
        assert_eq!(ev.poll_calls, 0);
    }
}
