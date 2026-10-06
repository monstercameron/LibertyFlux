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
    FIXED_REQUEST_A, FIXED_REQUEST_B, KIND_CLEAR, KIND_GATED_CONVERT, KIND_RESET_B,
    KIND_TYPE_CLEAR_B, ConvertRequest, EventChild, EventDispatch, EventHandler, EventPayload,
    EventRef, EventSource, FactoryAnswer, FactoryHandle, FactoryState, GatedAnswer, Owner, Task,
    TaskFactory, TaskManager,
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

fn manager(v: u32) -> Option<Handle32<TaskManager>> {
    Handle32::new(v)
}

#[test]
fn registry_counts_ten_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 10);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 41);
    assert_eq!(registry::ROWS.len(), 51);
    for row in registry::ROWS {
        assert_eq!(row.class, "EventHandler");
        if row.state == State::Proven {
            assert!(
                [
                    "vf14", "vf29", "vf31", "vf34", "vf36", "vf37", "vf54", "vf55", "vf67",
                    "vf68"
                ]
                .contains(&row.method),
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
    assert_eq!(KIND_GATED_CONVERT, 0x25C);
    assert_eq!(FIXED_REQUEST_A, 0x100);
    assert_eq!(FIXED_REQUEST_B, 0x200);
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

// The factory-pair slots.

/// Scripted factory: answers lookups and conversions from fixed words.
struct ScriptedFactory {
    lookup_answer: u32,
    convert_answer: u32,
    lookups: Vec<u32>,
    converts: Vec<(u32, ConvertRequest)>,
}

impl TaskFactory for ScriptedFactory {
    fn lookup(
        &mut self,
        manager: Option<Handle32<TaskManager>>,
    ) -> Option<Handle32<FactoryHandle>> {
        self.lookups.push(Handle32::raw_or_zero(manager));
        Handle32::new(self.lookup_answer)
    }

    fn convert(
        &mut self,
        handle: Handle32<FactoryHandle>,
        request: ConvertRequest,
    ) -> Option<Handle32<Task>> {
        self.converts.push((handle.get(), request));
        Handle32::new(self.convert_answer)
    }
}

fn factory(lookup_answer: u32, convert_answer: u32) -> ScriptedFactory {
    ScriptedFactory {
        lookup_answer,
        convert_answer,
        lookups: Vec::new(),
        converts: Vec::new(),
    }
}

#[test]
fn fixed_request_converts_its_code() {
    for code in [FIXED_REQUEST_A, FIXED_REQUEST_B] {
        let mut h = EventHandler::new(owner(1), task(7));
        let state = FactoryState::new(manager(0x3333));
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.answer_fixed_request(code, &mut f, &state),
            task(0x2222)
        );
        assert_eq!(h.pending(), task(0x2222));
        assert_eq!(f.lookups, vec![0x3333]);
        assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Code(code))]);
    }
}

#[test]
fn fixed_request_clears_when_no_handler_answers() {
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(0x3333));
    let mut f = factory(0, 0x2222);
    assert_eq!(h.answer_fixed_request(FIXED_REQUEST_A, &mut f, &state), None);
    assert_eq!(h.pending(), None);
    assert_eq!(f.lookups, vec![0x3333]);
    assert!(f.converts.is_empty());
}

#[test]
fn fixed_request_carries_a_null_manager() {
    // A null manager still reaches the lookup: the factory decides.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(None);
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_fixed_request(FIXED_REQUEST_A, &mut f, &state),
        task(0x2222)
    );
    assert_eq!(f.lookups, vec![0]);
}

#[test]
fn gated_type_takes_three_paths() {
    // Clear kind: clears, answers Cleared, never calls out.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_gated_type(KIND_CLEAR, &mut f, &state),
        GatedAnswer::Cleared
    );
    assert_eq!(h.pending(), None);
    assert!(f.lookups.is_empty());
    // Other kinds: untouched, answers Ignored, never calls out.
    for kind in [0, 0xC7, 0xC9, 0x25B, 0x25D, u32::MAX] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.answer_gated_type(kind, &mut f, &state),
            GatedAnswer::Ignored,
            "kind {kind:#x}"
        );
        assert_eq!(h.pending(), task(7));
        assert!(f.lookups.is_empty() && f.converts.is_empty());
    }
    // Convert kind: converts into the task slot.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_gated_type(KIND_GATED_CONVERT, &mut f, &state),
        GatedAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.lookups, vec![9]);
    assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Plain)]);
    // Convert kind with no handler: clears.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.answer_gated_type(KIND_GATED_CONVERT, &mut f, &state),
        GatedAnswer::Converted(None)
    );
    assert_eq!(h.pending(), None);
    assert!(f.converts.is_empty());
}

#[test]
fn child_passes_through_or_converts() {
    let event: Handle32<EventRef> = Handle32::new(0xE000).unwrap();
    // Null child: answers the event, task untouched, no calls.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_child(event, None, &mut f, &state),
        FactoryAnswer::Passthrough(event)
    );
    assert_eq!(h.pending(), task(7));
    assert!(f.lookups.is_empty());
    // Live child: converts it.
    let child: Handle32<EventChild> = Handle32::new(0xC44D).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_child(event, Some(child), &mut f, &state),
        FactoryAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.lookups, vec![9]);
    assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Child(child))]);
    // Live child with no handler: clears.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.answer_child(event, Some(child), &mut f, &state),
        FactoryAnswer::Converted(None)
    );
    assert_eq!(h.pending(), None);
}

#[test]
fn tagged_stages_identity_plus_tag() {
    // The staged word keeps the identity above the low byte.
    let event: Handle32<EventRef> = Handle32::new(0x1234_5600).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_tagged(0x777, 0xAB, event, &mut f, &state),
        task(0x2222)
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::Tagged {
                id: 0x777,
                staged: 0x1234_56AB
            }
        )]
    );
    // The low byte is replaced, not merged.
    let event: Handle32<EventRef> = Handle32::new(0x1234_56FF).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0x1111, 0x2222);
    let _ = h.answer_tagged(0, 0x00, event, &mut f, &state);
    assert_eq!(
        f.converts[0].1,
        ConvertRequest::Tagged {
            id: 0,
            staged: 0x1234_5600
        }
    );
    // No handler: clears without converting.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0, 0x2222);
    assert_eq!(h.answer_tagged(0x777, 0xAB, event, &mut f, &state), None);
    assert_eq!(h.pending(), None);
    assert!(f.converts.is_empty());
}

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
