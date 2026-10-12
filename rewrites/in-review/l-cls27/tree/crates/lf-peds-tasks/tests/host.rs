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
    BlockWords, ConvertRequest, EventChild, EventDispatch, EventHandler, EventLink, EventLinks,
    EventPayload, EventProbe, EventRef, EventSource, EventSubject, FIXED_REQUEST_A,
    FIXED_REQUEST_B, FactoryAnswer, FactoryHandle, FactoryState, FlaggedAnswer, GatedAnswer,
    GuardedAnswer, GuardedProbeAnswer, KIND_CLEAR, KIND_GATED_CONVERT, KIND_GUARDED_CONVERT,
    KIND_RESET_B, KIND_ROUTE_BUILD, KIND_ROUTED_CONVERT, KIND_TYPE_CLEAR_B, MARKER_MASK,
    MARKER_WANT, OWNER_REFRESH_FLAG, Owner, PROBE_EARLY_KIND, PROBE_LATE_KIND, ProbeInput,
    ProbeRegistry, ROUTE_BLOCK, RouteInput, RoutedAnswer, SETTLE_BUILD_KIND, ScalarEval,
    SettleStatus, SettledAnswer, StageHandle, StagedLookup, Task, TaskFactory, TaskManager,
    VecInput,
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
fn registry_counts_twenty_one_proven() {
    let (proven, lifted, missing) = registry::counts();
    assert_eq!(proven, 21);
    assert_eq!(lifted, 0);
    assert_eq!(missing, 33);
    assert_eq!(registry::ROWS.len(), 54);
    for row in registry::ROWS {
        assert_eq!(row.class, "EventHandler");
        if row.state == State::Proven {
            assert!(
                [
                    "vf14", "vf21", "vf23", "vf25", "vf28", "vf29", "vf31", "vf34", "vf35", "vf36",
                    "vf37", "vf41", "vf54", "vf55", "vf61", "vf63", "vf64", "vf67", "vf68", "vf71",
                    "vf73",
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

    fn dispatch_block(&mut self, _kind: u32, _event: Handle32<EventRef>, _block_offset: u32) {
        panic!("forward tests never dispatch blocks");
    }
}

#[test]
fn forward_clears_on_the_clear_kind_only() {
    let mut h = EventHandler::new(owner(1), task(7));
    let mut rec = Recorder { calls: Vec::new() };
    h.forward_or_clear(KIND_CLEAR, payload(0xFF), &mut rec);
    assert_eq!(h.pending(), None);
    assert!(rec.calls.is_empty());

    for (kind, pl) in [
        (0, None),
        (0xC7, payload(1)),
        (0xC9, None),
        (0x201, payload(2)),
    ] {
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

    fn probe_kind(&mut self) -> u32 {
        panic!("settle and adopt tests never probe kinds");
    }

    fn readiness(&mut self) -> u32 {
        panic!("settle and adopt tests never check readiness");
    }

    fn probe(&mut self) -> Option<Handle32<EventProbe>> {
        panic!("settle and adopt tests never probe");
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
        assert_eq!(h.answer_fixed_request(code, &mut f, &state), task(0x2222));
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
    assert_eq!(
        h.answer_fixed_request(FIXED_REQUEST_A, &mut f, &state),
        None
    );
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
fn refresh_flag_is_bit_two() {
    assert_eq!(OWNER_REFRESH_FLAG, 4);
}

fn subject(v: u32) -> Option<Handle32<EventSubject>> {
    Handle32::new(v)
}

// The refresh slots.

#[test]
fn mark_seen_sets_the_flag_and_converts() {
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    let mut seen = false;
    assert_eq!(
        h.refresh_marking_seen(subject(0x44), 0x55, &mut seen, &mut f, &state),
        task(0x2222)
    );
    assert!(seen);
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.lookups, vec![9]);
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::SubjectKind {
                subject: subject(0x44),
                kind: 0x55,
            }
        )]
    );
}

#[test]
fn mark_seen_marks_even_when_nothing_answers() {
    // The flag is set before the lookup, so it is set on every path.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0, 0x2222);
    let mut seen = false;
    assert_eq!(
        h.refresh_marking_seen(subject(0x44), 0x55, &mut seen, &mut f, &state),
        None
    );
    assert!(seen);
    assert_eq!(h.pending(), None);
    assert!(f.converts.is_empty());
}

#[test]
fn mark_seen_carries_a_null_subject() {
    // The slot never tests the subject word: null converts too.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    let mut seen = false;
    assert_eq!(
        h.refresh_marking_seen(None, 0x55, &mut seen, &mut f, &state),
        task(0x2222)
    );
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::SubjectKind {
                subject: None,
                kind: 0x55,
            }
        )]
    );
}

#[test]
fn subject_refresh_passes_through_or_converts() {
    let event: Handle32<EventRef> = Handle32::new(0xE000).unwrap();
    // Null subject: answers the event, task untouched, no calls.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_for_subject(event, None, &mut f, &state),
        FactoryAnswer::Passthrough(event)
    );
    assert_eq!(h.pending(), task(7));
    assert!(f.lookups.is_empty());
    // Live subject: converts it.
    let subj: Handle32<EventSubject> = Handle32::new(0x44).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_for_subject(event, Some(subj), &mut f, &state),
        FactoryAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.lookups, vec![9]);
    assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Subject(subj))]);
    // Live subject with no handler: clears.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.refresh_for_subject(event, Some(subj), &mut f, &state),
        FactoryAnswer::Converted(None)
    );
    assert_eq!(h.pending(), None);
}

#[test]
fn guarded_refresh_takes_three_paths() {
    let own: Handle32<Owner> = Handle32::new(0x111).unwrap();
    let event: Handle32<EventRef> = Handle32::new(0xE000).unwrap();
    let subj: Handle32<EventSubject> = Handle32::new(0x44).unwrap();
    let state = FactoryState::new(manager(9));
    // Flag set: keeps the task, answers the owner, never calls out,
    // even with a live subject.
    for flags in [4u8, 5, 6, 7, 0xFC, 0xFF] {
        let mut h = EventHandler::new(Some(own), task(7));
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.refresh_unless_owner_flagged(own, flags, event, Some(subj), &mut f, &state),
            GuardedAnswer::KeepOwner(own),
            "flags {flags:#x}"
        );
        assert_eq!(h.pending(), task(7));
        assert!(f.lookups.is_empty());
    }
    // Flag clear, null subject: keeps the task, answers the event.
    for flags in [0u8, 1, 2, 3, 0xFB] {
        let mut h = EventHandler::new(Some(own), task(7));
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.refresh_unless_owner_flagged(own, flags, event, None, &mut f, &state),
            GuardedAnswer::KeepEvent(event),
            "flags {flags:#x}"
        );
        assert_eq!(h.pending(), task(7));
        assert!(f.lookups.is_empty());
    }
    // Flag clear, live subject: converts.
    let mut h = EventHandler::new(Some(own), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_unless_owner_flagged(own, 0, event, Some(subj), &mut f, &state),
        GuardedAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Subject(subj))]);
}

#[test]
fn flagged_refresh_needs_a_live_flagged_owner() {
    let state = FactoryState::new(manager(9));
    // Null owner: keeps the task, answers NoOwner, never calls out.
    let mut h = EventHandler::new(None, task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_flagged_owner(0xFF, subject(1), 2, 3.0, &mut f, &state),
        FlaggedAnswer::NoOwner
    );
    assert_eq!(h.pending(), task(7));
    assert!(f.lookups.is_empty());
    // Live owner, flag clear: keeps the task, answers the owner.
    let own: Handle32<Owner> = Handle32::new(0x111).unwrap();
    for flags in [0u8, 1, 2, 3, 0xFB] {
        let mut h = EventHandler::new(Some(own), task(7));
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.refresh_flagged_owner(flags, subject(1), 2, 3.0, &mut f, &state),
            FlaggedAnswer::KeepOwner(own),
            "flags {flags:#x}"
        );
        assert_eq!(h.pending(), task(7));
        assert!(f.lookups.is_empty());
    }
    // Live owner, flag set: converts subject, kind and float bits.
    let mut h = EventHandler::new(Some(own), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_flagged_owner(4, subject(1), 2, 3.0, &mut f, &state),
        FlaggedAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::SubjectKindFloat {
                subject: subject(1),
                kind: 2,
                weight_bits: 3.0f32.to_bits(),
            }
        )]
    );
    // A null subject still converts on the flagged path.
    let mut h = EventHandler::new(Some(own), task(7));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_flagged_owner(4, None, 2, 3.0, &mut f, &state),
        FlaggedAnswer::Converted(task(0x2222))
    );
    assert_eq!(f.converts.len(), 1);
}

#[test]
fn flagged_refresh_carries_float_bits_exactly() {
    // The float word is carried bitwise: a NaN payload survives the trip.
    let own: Handle32<Owner> = Handle32::new(0x111).unwrap();
    let state = FactoryState::new(manager(9));
    let nan = f32::from_bits(0x7FC0_0001);
    let mut h = EventHandler::new(Some(own), task(7));
    let mut f = factory(0x1111, 0x2222);
    let _ = h.refresh_flagged_owner(4, subject(1), 2, nan, &mut f, &state);
    assert_eq!(
        f.converts[0].1,
        ConvertRequest::SubjectKindFloat {
            subject: subject(1),
            kind: 2,
            weight_bits: 0x7FC0_0001,
        }
    );
}

#[test]
fn build_response_builds_or_clears() {
    // Live allocation: builds into the task slot.
    let mut h = EventHandler::new(owner(1), task(7));
    let state = FactoryState::new(manager(9));
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(h.build_response(&mut f, &state), task(0x2222));
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(f.lookups, vec![9]);
    assert_eq!(f.converts, vec![(0x1111, ConvertRequest::Build)]);
    // Null allocation: clears without building.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut f = factory(0, 0x2222);
    assert_eq!(h.build_response(&mut f, &state), None);
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

// The new slots' constants.

#[test]
fn new_slots_have_their_values() {
    assert_eq!(KIND_ROUTE_BUILD, 0x398);
    assert_eq!(ROUTE_BLOCK, 0x20);
    assert_eq!(KIND_ROUTED_CONVERT, 0xE9);
    assert_eq!(PROBE_EARLY_KIND, 0x1B);
    assert_eq!(PROBE_LATE_KIND, 0x30);
    assert_eq!(KIND_GUARDED_CONVERT, 0x76C);
    assert_eq!(MARKER_MASK, 0x3C0);
    assert_eq!(MARKER_WANT, 0xC0);
    assert_eq!(SETTLE_BUILD_KIND, 5);
}

// The route slot.

/// Block-dispatch recorder for the route slot's host tests.
struct BlockRecorder {
    calls: Vec<(u32, u32, u32)>,
}

impl EventDispatch for BlockRecorder {
    fn dispatch_event(&mut self, _kind: u32, _payload: Option<Handle32<EventPayload>>) {
        panic!("route host tests never dispatch events");
    }

    fn dispatch_block(&mut self, kind: u32, event: Handle32<EventRef>, block_offset: u32) {
        self.calls.push((kind, event.get(), block_offset));
    }
}

fn route_input(kind: u32) -> RouteInput {
    RouteInput {
        kind,
        event: Handle32::new(0xE000).unwrap(),
        first: 1.5,
        second: -2.25,
    }
}

#[test]
fn route_stops_on_either_poll_shape() {
    let state = FactoryState::new(manager(9));
    // First poll null: one call, no store, no dispatch.
    for polls in [vec![None], vec![owner(0x44), owner(0x1111)]] {
        let mut h = EventHandler::new(owner(0x1111), task(7));
        let mut ev = Scripted {
            polls,
            clone_answer: None,
            poll_calls: 0,
            clone_calls: 0,
        };
        let mut d = BlockRecorder { calls: Vec::new() };
        let mut f = factory(0x1111, 0x2222);
        h.route_by_owner_and_kind(
            route_input(KIND_ROUTE_BUILD),
            &mut ev,
            &mut d,
            &mut f,
            &state,
        );
        assert_eq!(h.pending(), task(7));
        assert!(d.calls.is_empty());
        assert!(f.lookups.is_empty());
    }
    // Pass-through polls reach the kind dispatch.
    let mut h = EventHandler::new(owner(0x1111), task(7));
    let mut ev = Scripted {
        polls: vec![owner(0x44), owner(0x55)],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    let mut d = BlockRecorder { calls: Vec::new() };
    let mut f = factory(0x1111, 0x2222);
    h.route_by_owner_and_kind(route_input(0x1234), &mut ev, &mut d, &mut f, &state);
    assert_eq!(ev.poll_calls, 2);
    assert_eq!(d.calls, vec![(0x1234, 0xE000, ROUTE_BLOCK)]);
}

#[test]
fn route_clears_builds_or_dispatches_by_kind() {
    let state = FactoryState::new(manager(9));
    for kind in [KIND_CLEAR, KIND_RESET_B] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut ev = Scripted {
            polls: vec![owner(0x44), owner(0x55)],
            clone_answer: None,
            poll_calls: 0,
            clone_calls: 0,
        };
        let mut d = BlockRecorder { calls: Vec::new() };
        let mut f = factory(0x1111, 0x2222);
        h.route_by_owner_and_kind(route_input(kind), &mut ev, &mut d, &mut f, &state);
        assert_eq!(h.pending(), None, "kind {kind:#x}");
        assert!(d.calls.is_empty());
        assert!(f.lookups.is_empty());
    }
    // Neighbours of the clearing and build kinds dispatch instead.
    for kind in [0xC7, 0xC9, 0x397, 0x399, 0x3A6, 0x3A8] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut ev = Scripted {
            polls: vec![owner(0x44), owner(0x55)],
            clone_answer: None,
            poll_calls: 0,
            clone_calls: 0,
        };
        let mut d = BlockRecorder { calls: Vec::new() };
        let mut f = factory(0x1111, 0x2222);
        h.route_by_owner_and_kind(route_input(kind), &mut ev, &mut d, &mut f, &state);
        assert_eq!(h.pending(), task(7), "kind {kind:#x}");
        assert_eq!(d.calls, vec![(kind, 0xE000, ROUTE_BLOCK)], "kind {kind:#x}");
    }
    // The build kind converts the block and floats bitwise.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut ev = Scripted {
        polls: vec![owner(0x44), owner(0x55)],
        clone_answer: None,
        poll_calls: 0,
        clone_calls: 0,
    };
    let mut d = BlockRecorder { calls: Vec::new() };
    let mut f = factory(0x1111, 0x2222);
    h.route_by_owner_and_kind(
        route_input(KIND_ROUTE_BUILD),
        &mut ev,
        &mut d,
        &mut f,
        &state,
    );
    assert_eq!(h.pending(), task(0x2222));
    assert!(d.calls.is_empty());
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::BlockFloats {
                block_offset: ROUTE_BLOCK,
                first_bits: 1.5f32.to_bits(),
                second_bits: (-2.25f32).to_bits(),
            }
        )]
    );
}

// The dual-path float slot.

/// Scripted scalars for the float slots' host tests.
struct ScriptedScalar {
    word_answer: f32,
    block_answer: f32,
    words: Vec<u32>,
    blocks: Vec<(u32, BlockWords)>,
}

impl ScalarEval for ScriptedScalar {
    fn eval_word(&mut self, input: u32) -> f32 {
        self.words.push(input);
        self.word_answer
    }

    fn eval_block(&mut self, event: Handle32<EventRef>, words: BlockWords) -> f32 {
        self.blocks.push((event.get(), words));
        self.block_answer
    }
}

fn scalar(word: f32, block: f32) -> ScriptedScalar {
    ScriptedScalar {
        word_answer: word,
        block_answer: block,
        words: Vec::new(),
        blocks: Vec::new(),
    }
}

fn block_words() -> BlockWords {
    BlockWords {
        w34: 0x34,
        w30: 0x30,
        w3c: 0x3C,
        w40: 0x40,
    }
}

#[test]
fn scalar_block_picks_its_converter_by_bit() {
    let state = FactoryState::new(manager(9));
    let event = Handle32::new(0xE000).unwrap();
    // Bit set takes the first sibling, clear the second; the other
    // seven bits change nothing.
    for selector in 0..=0xFFu8 {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut s = scalar(0.0, 2.5);
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.answer_scalar_block(selector, event, block_words(), &mut s, &mut f, &state),
            task(0x2222),
            "selector {selector:#x}"
        );
        let want_second = selector & 2 == 0;
        assert_eq!(
            f.converts,
            vec![(
                0x1111,
                ConvertRequest::ScalarBlock {
                    scalar_bits: 2.5f32.to_bits(),
                    words: block_words(),
                    second: want_second,
                }
            )],
            "selector {selector:#x}"
        );
        assert_eq!(s.blocks, vec![(0xE000, block_words())]);
    }
}

#[test]
fn scalar_block_skips_the_eval_when_no_handler_answers() {
    let state = FactoryState::new(manager(9));
    let event = Handle32::new(0xE000).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let mut s = scalar(0.0, 2.5);
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.answer_scalar_block(0, event, block_words(), &mut s, &mut f, &state),
        None
    );
    assert_eq!(h.pending(), None);
    assert!(s.blocks.is_empty());
    assert!(f.converts.is_empty());
}

// The routed-probe slot.

/// Scripted probes and readiness for the routed slot's host tests.
struct Probed {
    kinds: Vec<u32>,
    kind_calls: usize,
    ready_answer: u32,
    ready_calls: usize,
}

impl EventSource for Probed {
    fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
        panic!("routed host tests never poll");
    }

    fn clone_task(&mut self) -> Option<Handle32<Task>> {
        panic!("routed host tests never clone");
    }

    fn probe_kind(&mut self) -> u32 {
        let i = self.kind_calls.min(self.kinds.len().saturating_sub(1));
        self.kind_calls += 1;
        self.kinds[i]
    }

    fn readiness(&mut self) -> u32 {
        self.ready_calls += 1;
        self.ready_answer
    }

    fn probe(&mut self) -> Option<Handle32<EventProbe>> {
        panic!("routed host tests never probe");
    }
}

/// Scripted two-stage lookup for the routed slot's host tests.
struct ScriptedLookup {
    stage_answer: u32,
    finish_answer: u32,
    stages: u32,
    finishes: u32,
}

impl StagedLookup for ScriptedLookup {
    fn stage(
        &mut self,
        _primary: Option<Handle32<EventLink>>,
        _secondary: Option<Handle32<EventLink>>,
    ) -> Option<Handle32<StageHandle>> {
        self.stages += 1;
        Handle32::new(self.stage_answer)
    }

    fn finish(
        &mut self,
        _staged: Option<Handle32<StageHandle>>,
        _primary: Option<Handle32<EventLink>>,
        _secondary: Option<Handle32<EventLink>>,
    ) -> u32 {
        self.finishes += 1;
        self.finish_answer
    }
}

fn links(primary: u32, secondary: u32) -> EventLinks {
    EventLinks {
        primary: Handle32::new(primary),
        secondary: Handle32::new(secondary),
    }
}

#[test]
fn routed_probe_gates_each_path() {
    let state = FactoryState::new(manager(9));
    // Early probe with a null secondary link stops at once.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![PROBE_EARLY_KIND],
        kind_calls: 0,
        ready_answer: 0xFF,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 1,
        finish_answer: 1,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(links(5, 0), 0xE9, &mut e, &mut l, &mut f, &state),
        RoutedAnswer::EarlyProbe(PROBE_EARLY_KIND)
    );
    assert_eq!(e.kind_calls, 1);
    assert_eq!(e.ready_calls, 0);
    assert_eq!(h.pending(), task(7));
    // A live secondary link sails past the early probe.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![PROBE_EARLY_KIND, PROBE_LATE_KIND],
        kind_calls: 0,
        ready_answer: 1,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 1,
        finish_answer: 1,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(links(5, 6), 0xE9, &mut e, &mut l, &mut f, &state),
        RoutedAnswer::EarlyProbe(PROBE_LATE_KIND)
    );
    assert_eq!(e.kind_calls, 2);
    // A clear low byte on the readiness check skips the second probe.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![0x99, PROBE_LATE_KIND],
        kind_calls: 0,
        ready_answer: 0x100,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 1,
        finish_answer: 1,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(links(5, 6), KIND_CLEAR, &mut e, &mut l, &mut f, &state),
        RoutedAnswer::StoredNull
    );
    assert_eq!(e.kind_calls, 1);
    assert_eq!(h.pending(), None);
}

#[test]
fn routed_probe_answers_unrouted_kinds_by_wrapping_sub() {
    let state = FactoryState::new(manager(9));
    // Below the convert kind the answer wraps around.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![0x99],
        kind_calls: 0,
        ready_answer: 0,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 1,
        finish_answer: 1,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(links(0, 0), 0, &mut e, &mut l, &mut f, &state),
        RoutedAnswer::Unrouted(0u32.wrapping_sub(KIND_ROUTED_CONVERT))
    );
    assert_eq!(h.pending(), task(7));
    assert_eq!(l.stages, 0);
}

#[test]
fn routed_probe_runs_the_lookup_before_the_factory() {
    let state = FactoryState::new(manager(9));
    // A failed second stage stores null without touching the factory.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![0x99],
        kind_calls: 0,
        ready_answer: 0,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 0xBEEF,
        finish_answer: 0x500,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(
            links(5, 6),
            KIND_ROUTED_CONVERT,
            &mut e,
            &mut l,
            &mut f,
            &state
        ),
        RoutedAnswer::LookupFailed(0x500)
    );
    assert_eq!(h.pending(), None);
    assert!(f.lookups.is_empty());
    // A passing stage converts the pair.
    let mut h = EventHandler::new(owner(1), task(7));
    let mut e = Probed {
        kinds: vec![0x99],
        kind_calls: 0,
        ready_answer: 0,
        ready_calls: 0,
    };
    let mut l = ScriptedLookup {
        stage_answer: 0,
        finish_answer: 0x501,
        stages: 0,
        finishes: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_routed_probe(
            links(5, 6),
            KIND_ROUTED_CONVERT,
            &mut e,
            &mut l,
            &mut f,
            &state
        ),
        RoutedAnswer::Converted(task(0x2222))
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(
        f.converts,
        vec![(
            0x1111,
            ConvertRequest::Pair {
                primary: Handle32::new(5),
                secondary: Handle32::new(6),
            }
        )]
    );
}

// The guarded-probe slot.

/// Scripted probe event for the guarded slot's host tests.
struct ProbeEvent {
    answer: u32,
    calls: u32,
}

impl EventSource for ProbeEvent {
    fn poll_owner(&mut self) -> Option<Handle32<Owner>> {
        panic!("guarded host tests never poll");
    }

    fn clone_task(&mut self) -> Option<Handle32<Task>> {
        panic!("guarded host tests never clone");
    }

    fn probe_kind(&mut self) -> u32 {
        panic!("guarded host tests never probe kinds");
    }

    fn readiness(&mut self) -> u32 {
        panic!("guarded host tests never check readiness");
    }

    fn probe(&mut self) -> Option<Handle32<EventProbe>> {
        self.calls += 1;
        Handle32::new(self.answer)
    }
}

/// Scripted registry for the guarded slot's host tests.
struct ScriptedRegistry {
    check_answer: u32,
    release_answer: u32,
    checks: u32,
    releases: u32,
}

impl ProbeRegistry for ScriptedRegistry {
    fn check(&mut self, _owner: Handle32<Owner>, _kind: u32) -> u32 {
        self.checks += 1;
        self.check_answer
    }

    fn release(&mut self, _owner: Handle32<Owner>, _probe: Handle32<EventProbe>) -> u32 {
        self.releases += 1;
        self.release_answer
    }
}

fn guarded_probe(kind: u32, status: u32) -> ProbeInput {
    ProbeInput { kind, status }
}

#[test]
fn guarded_probe_answers_the_release_not_the_conversion() {
    let state = FactoryState::new(manager(9));
    let owner_h = Handle32::new(0xB00).unwrap();
    // The conversion lands in the task slot; the answer is the release's.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut e = ProbeEvent {
        answer: 0xF00,
        calls: 0,
    };
    let mut r = ScriptedRegistry {
        check_answer: 0,
        release_answer: 0xDEC,
        checks: 0,
        releases: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_guarded_probe(
            owner_h,
            guarded_probe(KIND_GUARDED_CONVERT, MARKER_WANT),
            &mut e,
            &mut r,
            &mut f,
            &state
        ),
        GuardedProbeAnswer::Converted {
            task: task(0x2222),
            released: 0xDEC,
        }
    );
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(r.releases, 1);
    // The release runs even when no handler answers.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut e = ProbeEvent {
        answer: 0xF00,
        calls: 0,
    };
    let mut r = ScriptedRegistry {
        check_answer: 0,
        release_answer: 0xDEC,
        checks: 0,
        releases: 0,
    };
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.answer_guarded_probe(
            owner_h,
            guarded_probe(KIND_GUARDED_CONVERT, MARKER_WANT | 0xFFFF_FC00),
            &mut e,
            &mut r,
            &mut f,
            &state
        ),
        GuardedProbeAnswer::Converted {
            task: None,
            released: 0xDEC,
        }
    );
    assert_eq!(h.pending(), None);
    assert_eq!(r.releases, 1);
    assert!(f.converts.is_empty());
}

#[test]
fn guarded_probe_rejects_each_gate() {
    let state = FactoryState::new(manager(9));
    let owner_h = Handle32::new(0xB00).unwrap();
    // A nonzero registry answer ends the call before any gate.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut e = ProbeEvent {
        answer: 0xF00,
        calls: 0,
    };
    let mut r = ScriptedRegistry {
        check_answer: 0x77,
        release_answer: 0,
        checks: 0,
        releases: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_guarded_probe(
            owner_h,
            guarded_probe(KIND_GUARDED_CONVERT, MARKER_WANT),
            &mut e,
            &mut r,
            &mut f,
            &state
        ),
        GuardedProbeAnswer::Registry(0x77)
    );
    assert_eq!(h.pending(), task(7));
    assert_eq!(r.releases, 0);
    assert!(f.lookups.is_empty());
    // Wrong kind and null probe both reject with zero.
    for (kind, probe) in [(0x76B, 0xF00), (KIND_GUARDED_CONVERT, 0)] {
        let mut h = EventHandler::new(owner(0xB00), task(7));
        let mut e = ProbeEvent {
            answer: probe,
            calls: 0,
        };
        let mut r = ScriptedRegistry {
            check_answer: 0,
            release_answer: 0,
            checks: 0,
            releases: 0,
        };
        let mut f = factory(0x1111, 0x2222);
        assert_eq!(
            h.answer_guarded_probe(
                owner_h,
                guarded_probe(kind, MARKER_WANT),
                &mut e,
                &mut r,
                &mut f,
                &state
            ),
            GuardedProbeAnswer::Rejected(0),
            "kind {kind:#x} probe {probe:#x}"
        );
        assert_eq!(h.pending(), task(7));
        assert_eq!(r.releases, 0);
    }
    // Failed marker bits reject with the masked word.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut e = ProbeEvent {
        answer: 0xF00,
        calls: 0,
    };
    let mut r = ScriptedRegistry {
        check_answer: 0,
        release_answer: 0,
        checks: 0,
        releases: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.answer_guarded_probe(
            owner_h,
            guarded_probe(KIND_GUARDED_CONVERT, 0xFFFF_FD80),
            &mut e,
            &mut r,
            &mut f,
            &state
        ),
        GuardedProbeAnswer::Rejected(0xFFFF_FD80 & MARKER_MASK)
    );
    assert_eq!(h.pending(), task(7));
    assert_eq!(r.releases, 0);
}

// The scalar-vector slot.

#[test]
fn scalar_vec_picks_its_base_by_flag() {
    let state = FactoryState::new(manager(9));
    let event = Handle32::new(0xE000).unwrap();
    // Only flag 1 takes the first base; neighbours take the second.
    for (flag, want) in [(1u32, 0x10u32), (0, 0x20), (2, 0x20), (u32::MAX, 0x20)] {
        let mut h = EventHandler::new(owner(1), task(7));
        let mut s = scalar(3.75, 0.0);
        let mut f = factory(0x1111, 0x2222);
        let input = VecInput {
            flag,
            scalar_input: 0xAB,
            weight: -0.5,
        };
        assert_eq!(
            h.answer_scalar_vec(input, event, &mut s, &mut f, &state),
            task(0x2222),
            "flag {flag:#x}"
        );
        assert_eq!(s.words, vec![0xAB]);
        assert_eq!(
            f.converts,
            vec![(
                0x1111,
                ConvertRequest::ScalarVec {
                    scalar_bits: 3.75f32.to_bits(),
                    vec_offset: want,
                    weight_bits: (-0.5f32).to_bits(),
                }
            )],
            "flag {flag:#x}"
        );
    }
}

#[test]
fn scalar_vec_evaluates_before_the_lookup() {
    // Unlike its dual-path sibling, the scalar call runs even when no
    // handler answers.
    let state = FactoryState::new(manager(9));
    let event = Handle32::new(0xE000).unwrap();
    let mut h = EventHandler::new(owner(1), task(7));
    let mut s = scalar(3.75, 0.0);
    let mut f = factory(0, 0x2222);
    let input = VecInput {
        flag: 1,
        scalar_input: 0xAB,
        weight: 1.0,
    };
    assert_eq!(
        h.answer_scalar_vec(input, event, &mut s, &mut f, &state),
        None
    );
    assert_eq!(h.pending(), None);
    assert_eq!(s.words, vec![0xAB]);
    assert!(f.converts.is_empty());
}

// The settle slot.

/// Scripted settle status for the settle slot's host tests.
struct ScriptedSettle {
    ready_answer: u32,
    check_answer: u32,
    readies: u32,
    checks: u32,
}

impl SettleStatus for ScriptedSettle {
    fn owner_ready(&mut self, _owner: Handle32<Owner>) -> u32 {
        self.readies += 1;
        self.ready_answer
    }

    fn confirm_settled(&mut self) -> u32 {
        self.checks += 1;
        self.check_answer
    }
}

#[test]
fn settled_tests_only_low_bytes() {
    let state = FactoryState::new(manager(9));
    let owner_h = Handle32::new(0xB00).unwrap();
    // A set low byte on both answers settles with the check's word.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut s = ScriptedSettle {
        ready_answer: 0xDEAD_0001,
        check_answer: 0xBEEF_00FF,
        readies: 0,
        checks: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_unless_settled(owner_h, &mut s, &mut f, &state),
        SettledAnswer::Settled(0xBEEF_00FF)
    );
    assert_eq!(h.pending(), task(7));
    assert!(f.lookups.is_empty());
    // A clear low byte on the ready word skips the check entirely.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut s = ScriptedSettle {
        ready_answer: 0xFFFF_FF00,
        check_answer: 0xFF,
        readies: 0,
        checks: 0,
    };
    let mut f = factory(0x1111, 0x2222);
    assert_eq!(
        h.refresh_unless_settled(owner_h, &mut s, &mut f, &state),
        SettledAnswer::Refreshed(task(0x2222))
    );
    assert_eq!(s.checks, 0);
    assert_eq!(h.pending(), task(0x2222));
    assert_eq!(
        f.converts,
        vec![(0x1111, ConvertRequest::Code(SETTLE_BUILD_KIND))]
    );
    // A clear low byte on the check converts instead of settling.
    let mut h = EventHandler::new(owner(0xB00), task(7));
    let mut s = ScriptedSettle {
        ready_answer: 1,
        check_answer: 0x1234_5600,
        readies: 0,
        checks: 0,
    };
    let mut f = factory(0, 0x2222);
    assert_eq!(
        h.refresh_unless_settled(owner_h, &mut s, &mut f, &state),
        SettledAnswer::Refreshed(None)
    );
    assert_eq!(s.checks, 1);
    assert_eq!(h.pending(), None);
}
