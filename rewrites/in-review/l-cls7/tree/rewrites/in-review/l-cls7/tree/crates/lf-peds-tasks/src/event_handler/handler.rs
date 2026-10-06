//! The lifted event handler: one owning type for `CEventHandler`.
//!
//! The original keeps a small handler object per pedestrian: an owner
//! pointer just past the virtual table and a pending-task slot the event
//! slots clear, forward or fill. This type owns those two words as
//! ordinary Rust data (opaque identities, never addresses) and restates
//! each verified 32-bit method with behaviour in it as a method.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-evthandler-diff` test crate). Nothing here is
//! verified by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`].

use lf_core::Handle32;

/// The object an event handler answers for (its owner word).
///
/// Opaque: the lifted handler compares it and hands it back across the
/// boundary, never interprets it. It becomes a real handle when its
/// owner class lifts.
#[derive(Debug)]
pub struct Owner;

/// A pending task held by the handler (its task slot).
///
/// Opaque for the same reason as [`Owner`].
#[derive(Debug)]
pub struct Task;

/// The payload word an event carries into the dispatch slot.
///
/// Opaque: the slot the handler forwards to interprets it, the handler
/// only carries it through.
#[derive(Debug)]
pub struct EventPayload;

/// An event's identity (its address on the 32-bit side).
///
/// Opaque: the child slot answers the event itself when there is
/// nothing to convert, and the tagged slot folds the identity's bits
/// into its staged request word.
#[derive(Debug)]
pub struct EventRef;

/// The child word an event carries into the child slot.
///
/// Opaque: the factory interprets it, the handler only tests it for
/// null and carries it through.
#[derive(Debug)]
pub struct EventChild;

/// The subject word an event carries into the refresh slots.
///
/// Opaque: the factory interprets it, the handler only tests it for
/// null and carries it through.
#[derive(Debug)]
pub struct EventSubject;

/// The probe answer an event carries out of its probe slot.
///
/// Opaque: the guarded slot tests it for null, reads its status word
/// (which travels separately) and carries it through the conversion
/// and the registry release.
#[derive(Debug)]
pub struct EventProbe;

/// One of the link words an event carries into the routed-probe slot.
///
/// Opaque: the slot tests the secondary link for null and carries
/// both through the staged lookup and the conversion.
#[derive(Debug)]
pub struct EventLink;

/// The intermediate answer of the routed-probe slot's first stage.
///
/// Opaque: carried into the second stage, never interpreted.
#[derive(Debug)]
pub struct StageHandle;

/// Event kind that clears the pending task wherever it is tested: the
/// type-clear slot, the payload-gated reset slot and the forward-or-clear
/// slot all treat this kind as "drop the task".
pub const KIND_CLEAR: u32 = 0xC8;

/// Second clearing kind of the type-clear slot only.
pub const KIND_TYPE_CLEAR_B: u32 = 0x201;

/// Second reset kind of the payload-gated slot only.
pub const KIND_RESET_B: u32 = 0x3A7;

/// Fixed request code the first fixed-request slot converts.
pub const FIXED_REQUEST_A: u32 = 0x100;

/// Fixed request code the second fixed-request slot converts.
pub const FIXED_REQUEST_B: u32 = 0x200;

/// The kind the type-gated slot converts through the factory.
pub const KIND_GATED_CONVERT: u32 = 0x25C;

/// The kind the route slot builds through the factory.
pub const KIND_ROUTE_BUILD: u32 = 0x398;

/// Byte offset of the event block the route slot builds and dispatches.
pub const ROUTE_BLOCK: u32 = 0x20;

/// Selector bit picking the dual-path slot's converter: set takes the
/// first sibling, clear the second.
pub const BLOCK_SELECT_BIT: u8 = 2;

/// The kind the routed-probe slot converts through the factory.
pub const KIND_ROUTED_CONVERT: u32 = 0xE9;

/// The kind the routed-probe slot's first probe stops on (with a null
/// secondary link).
pub const PROBE_EARLY_KIND: u32 = 0x1B;

/// The kind the routed-probe slot's second probe stops on (when the
/// readiness check's low byte is set).
pub const PROBE_LATE_KIND: u32 = 0x30;

/// The kind the guarded-probe slot converts through the factory.
pub const KIND_GUARDED_CONVERT: u32 = 0x76C;

/// Mask selecting the marker bits in a probe's status word.
pub const MARKER_MASK: u32 = 0x3C0;

/// The marker bits a probe's status word must carry to convert.
pub const MARKER_WANT: u32 = 0xC0;

/// Flag word picking the scalar-vector slot's first vector base.
pub const VEC_FIRST_FLAG: u32 = 1;

/// Byte offset of the scalar-vector slot's first vector base.
pub const VEC_FIRST_BASE: u32 = 0x10;

/// Byte offset of the scalar-vector slot's second vector base.
pub const VEC_SECOND_BASE: u32 = 0x20;

/// The request code the settle slot converts through the factory.
pub const SETTLE_BUILD_KIND: u32 = 5;

/// Owner flag bit that gates the owner-reading refresh slots.
///
/// The guarded slot keeps its task when the owner's flag byte carries
/// this bit; the flagged slot refreshes its task only then.
pub const OWNER_REFRESH_FLAG: u8 = 4;

/// What the handler calls back on itself: its own dispatch slots.
///
/// The forward-or-clear slot answers an event it does not clear by
/// calling the event dispatch with the event's kind and payload words;
/// the route slot answers an unbuilt kind by calling the block dispatch
/// with the kind and the event's block.
pub trait EventDispatch {
    /// Handles one forwarded event.
    fn dispatch_event(&mut self, kind: u32, payload: Option<Handle32<EventPayload>>);

    /// Handles one routed event block: the kind and the event block at
    /// the given byte offset from the event.
    fn dispatch_block(&mut self, kind: u32, event: Handle32<EventRef>, block_offset: u32);
}

/// What the handler calls on an event: the event-side collaborator.
///
/// The settle slot polls the event's owner until it settles; the adopt
/// slot asks the event to clone its task; the routed-probe slot reads
/// the event's kind probe and readiness; the guarded-probe slot reads
/// its probe. Production code implements this on the lifted event;
/// tests implement it on fakes.
pub trait EventSource {
    /// Answers the event's owner word (the settle slot's poll).
    fn poll_owner(&mut self) -> Option<Handle32<Owner>>;

    /// Clones the event's task (the adopt slot's conversion).
    fn clone_task(&mut self) -> Option<Handle32<Task>>;

    /// Answers the event's kind probe (the routed-probe slot's gate).
    fn probe_kind(&mut self) -> u32;

    /// Answers the event's readiness word (only the low byte is tested).
    fn readiness(&mut self) -> u32;

    /// Answers the event's probe (the guarded-probe slot's conversion).
    fn probe(&mut self) -> Option<Handle32<EventProbe>>;
}

/// What the handler asks of the float-scalar calls: the scalar-side
/// collaborator.
///
/// The scalar slots evaluate one float from event words before
/// converting; the value travels bit-exact into the conversion.
/// Production code implements this on the lifted scalar routines;
/// tests pass a fake that records calls and scripts answers.
pub trait ScalarEval {
    /// Evaluates a scalar from one event word.
    fn eval_word(&mut self, input: u32) -> f32;

    /// Evaluates a scalar from the event's vector block.
    fn eval_block(&mut self, event: Handle32<EventRef>, words: BlockWords) -> f32;
}

/// What the handler asks of the routed-probe slot's two-stage lookup.
///
/// The first stage reduces the link pair to an intermediate answer;
/// the second stage turns it into a word whose low byte gates the
/// conversion. Tests pass a fake that records calls and scripts
/// answers.
pub trait StagedLookup {
    /// Runs the first stage on the link pair.
    fn stage(
        &mut self,
        primary: Option<Handle32<EventLink>>,
        secondary: Option<Handle32<EventLink>>,
    ) -> Option<Handle32<StageHandle>>;

    /// Runs the second stage on the intermediate answer and the pair.
    fn finish(
        &mut self,
        staged: Option<Handle32<StageHandle>>,
        primary: Option<Handle32<EventLink>>,
        secondary: Option<Handle32<EventLink>>,
    ) -> u32;
}

/// What the handler asks of the probe registry: the registry-side
/// collaborator.
///
/// The guarded-probe slot checks each kind against the registry and
/// releases every converted probe; the release's answer is the slot's
/// answer. Tests pass a fake that records calls and scripts answers.
pub trait ProbeRegistry {
    /// Checks a kind against the registry: nonzero ends the slot.
    fn check(&mut self, owner: Handle32<Owner>, kind: u32) -> u32;

    /// Releases a converted probe; the answer is the slot's answer.
    fn release(&mut self, owner: Handle32<Owner>, probe: Handle32<EventProbe>) -> u32;
}

/// What the handler asks about settling: the owner-side collaborator.
///
/// The settle slot asks the owner's ready slot and, when ready,
/// confirms with a follow-up check; only the low byte of each answer
/// is tested. Tests pass a fake that records calls and scripts
/// answers.
pub trait SettleStatus {
    /// Asks the owner's ready slot (only the low byte is tested).
    fn owner_ready(&mut self, owner: Handle32<Owner>) -> u32;

    /// Runs the follow-up check (only the low byte is tested).
    fn confirm_settled(&mut self) -> u32;
}

/// A pedestrian's event handler: its owner and its pending task.
///
/// The 32-bit object holds these two words past its virtual table; the
/// lifted form owns them directly. A missing task (`None`) is the
/// original's null task word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventHandler {
    /// The owner word: compared by the settle slot, never dereferenced.
    owner: Option<Handle32<Owner>>,
    /// The pending-task slot: cleared, forwarded or filled by the slots.
    pending: Option<Handle32<Task>>,
}

impl EventHandler {
    /// Builds a handler from its two words.
    #[must_use]
    pub const fn new(owner: Option<Handle32<Owner>>, pending: Option<Handle32<Task>>) -> Self {
        Self { owner, pending }
    }

    /// The owner word.
    #[must_use]
    pub const fn owner(self) -> Option<Handle32<Owner>> {
        self.owner
    }

    /// The pending task (`None` is the original's null).
    #[must_use]
    pub const fn pending(self) -> Option<Handle32<Task>> {
        self.pending
    }

    /// Type-clear slot: drops the pending task for two clearing kinds.
    ///
    /// Restates the verified slot that reads the event's kind word and
    /// clears the task slot when the kind is [`KIND_CLEAR`] or
    /// [`KIND_TYPE_CLEAR_B`], leaving it alone otherwise. Answers the
    /// kind word unchanged either way.
    pub fn clear_pending_on_type(&mut self, kind: u32) -> u32 {
        if kind == KIND_CLEAR || kind == KIND_TYPE_CLEAR_B {
            self.pending = None;
        }
        kind
    }

    /// Payload-gated reset slot: drops the pending task for two kinds.
    ///
    /// Restates the verified slot that does nothing when the event
    /// carries no payload and otherwise clears the task slot when the
    /// kind is [`KIND_CLEAR`] or [`KIND_RESET_B`]. The original answers a
    /// constant zero, which carries no meaning and is not modelled.
    pub fn reset_pending_on_kind(&mut self, kind: u32, payload_present: bool) {
        if !payload_present {
            return;
        }
        if kind == KIND_CLEAR || kind == KIND_RESET_B {
            self.pending = None;
        }
    }

    /// Forward-or-clear slot: clears the task or forwards the event.
    ///
    /// Restates the verified slot that clears the task slot when the
    /// kind is [`KIND_CLEAR`] and otherwise calls the handler's own
    /// dispatch slot with the kind and payload words. The original
    /// answers a constant zero, which carries no meaning.
    pub fn forward_or_clear(
        &mut self,
        kind: u32,
        payload: Option<Handle32<EventPayload>>,
        dispatch: &mut impl EventDispatch,
    ) {
        if kind == KIND_CLEAR {
            self.pending = None;
            return;
        }
        dispatch.dispatch_event(kind, payload);
    }

    /// Settle slot: polls the event's owner until it settles.
    ///
    /// Restates the verified slot that polls the event once and stops
    /// when the answer is null, polls again and stops when the answer
    /// equals this handler's owner, and otherwise polls a third and
    /// final time. Makes one to three identical calls and no stores; the
    /// original answers a constant zero, which carries no meaning.
    pub fn settle_poll(&self, event: &mut impl EventSource) {
        if event.poll_owner().is_none() {
            return;
        }
        if event.poll_owner() == self.owner {
            return;
        }
        event.poll_owner();
    }

    /// Adopt slot: replaces the pending task with the event's clone.
    ///
    /// Restates the verified slot that asks the event to clone its task,
    /// stores the answer in the task slot and answers it as well.
    pub fn adopt_cloned_task(&mut self, event: &mut impl EventSource) -> Option<Handle32<Task>> {
        let task = event.clone_task();
        self.pending = task;
        task
    }

    /// Fixed-request slot: converts one constant request through the factory.
    ///
    /// Restates the two verified slots that ignore their arguments, look
    /// up a conversion handler from the shared manager and convert their
    /// fixed code ([`FIXED_REQUEST_A`] for the first, [`FIXED_REQUEST_B`]
    /// for the second), storing the answer in the task slot. When no
    /// handler answers, the task slot is cleared and no conversion runs.
    pub fn answer_fixed_request(
        &mut self,
        code: u32,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let answer = factory.convert(handle, ConvertRequest::Code(code));
        self.pending = answer;
        answer
    }

    /// Type-gated slot: clears, ignores or converts by kind.
    ///
    /// Restates the verified slot that reads the event's kind word:
    /// [`KIND_CLEAR`] clears the task slot, any kind other than
    /// [`KIND_GATED_CONVERT`] leaves it alone, and the convert kind
    /// converts through the factory into the task slot (clearing it
    /// when no handler answers). Answers the kind word on the first
    /// two paths and the conversion on the third.
    pub fn answer_gated_type(
        &mut self,
        kind: u32,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> GatedAnswer {
        if kind == KIND_CLEAR {
            self.pending = None;
            return GatedAnswer::Cleared;
        }
        if kind != KIND_GATED_CONVERT {
            return GatedAnswer::Ignored;
        }
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return GatedAnswer::Converted(None);
        };
        let answer = factory.convert(handle, ConvertRequest::Plain);
        self.pending = answer;
        GatedAnswer::Converted(answer)
    }

    /// Child slot: passes the event through or converts its child.
    ///
    /// Restates the verified slot that answers the event itself when
    /// its child word is null and otherwise converts the child through
    /// the factory into the task slot (clearing the slot when no
    /// handler answers). The task slot is untouched on the
    /// pass-through path.
    pub fn answer_child(
        &mut self,
        event: Handle32<EventRef>,
        child: Option<Handle32<EventChild>>,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> FactoryAnswer {
        let Some(child) = child else {
            return FactoryAnswer::Passthrough(event);
        };
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return FactoryAnswer::Converted(None);
        };
        let answer = factory.convert(handle, ConvertRequest::Child(child));
        self.pending = answer;
        FactoryAnswer::Converted(answer)
    }

    /// Tagged slot: converts an id with a tag-staged word.
    ///
    /// Restates the verified slot that reads the event's id word and
    /// tag byte, stages a request word from the event identity with
    /// its low byte replaced by the tag, and converts the pair through
    /// the factory into the task slot (clearing the slot when no
    /// handler answers). Answers the conversion either way.
    pub fn answer_tagged(
        &mut self,
        id: u32,
        tag: u8,
        event: Handle32<EventRef>,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let staged = (event.get() & 0xFFFF_FF00) | u32::from(tag);
        let answer = factory.convert(handle, ConvertRequest::Tagged { id, staged });
        self.pending = answer;
        answer
    }

    /// Mark-seen refresh slot: marks the event seen, converts subject and kind.
    ///
    /// Restates the verified slot that sets the event's seen flag,
    /// looks up a conversion handler from the shared manager and
    /// converts the event's subject and kind words into the task slot
    /// (clearing the slot when no handler answers). Answers the
    /// conversion either way. The subject word is carried through even
    /// when null: the slot never tests it.
    pub fn refresh_marking_seen(
        &mut self,
        subject: Option<Handle32<EventSubject>>,
        kind: u32,
        seen: &mut bool,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        *seen = true;
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let answer = factory.convert(handle, ConvertRequest::SubjectKind { subject, kind });
        self.pending = answer;
        answer
    }

    /// Subject refresh slot: passes the event through or converts its subject.
    ///
    /// Restates the verified slot that answers the event itself when
    /// its subject word is null (leaving the task slot alone) and
    /// otherwise converts the subject through the factory into the
    /// task slot (clearing the slot when no handler answers).
    pub fn refresh_for_subject(
        &mut self,
        event: Handle32<EventRef>,
        subject: Option<Handle32<EventSubject>>,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> FactoryAnswer {
        let Some(subject) = subject else {
            return FactoryAnswer::Passthrough(event);
        };
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return FactoryAnswer::Converted(None);
        };
        let answer = factory.convert(handle, ConvertRequest::Subject(subject));
        self.pending = answer;
        FactoryAnswer::Converted(answer)
    }

    /// Owner-gated refresh slot: keeps the task or converts the subject.
    ///
    /// Restates the verified slot that answers the owner word when the
    /// owner's flag byte carries [`OWNER_REFRESH_FLAG`], answers the
    /// event when its subject word is null, and otherwise converts the
    /// subject through the factory into the task slot (clearing the
    /// slot when no handler answers). The task slot is untouched on
    /// the first two paths. The owner word travels as a parameter
    /// rather than from this handler because the slot reads it but
    /// faults unless it is live, which the non-optional handle states
    /// in the type.
    pub fn refresh_unless_owner_flagged(
        &mut self,
        owner: Handle32<Owner>,
        owner_flags: u8,
        event: Handle32<EventRef>,
        subject: Option<Handle32<EventSubject>>,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> GuardedAnswer {
        if owner_flags & OWNER_REFRESH_FLAG != 0 {
            return GuardedAnswer::KeepOwner(owner);
        }
        let Some(subject) = subject else {
            return GuardedAnswer::KeepEvent(event);
        };
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return GuardedAnswer::Converted(None);
        };
        let answer = factory.convert(handle, ConvertRequest::Subject(subject));
        self.pending = answer;
        GuardedAnswer::Converted(answer)
    }

    /// Flagged-owner refresh slot: refreshes only for a flagged owner.
    ///
    /// Restates the verified slot that keeps its task and answers zero
    /// when the owner word is null, keeps its task and answers the
    /// owner when the owner's flag byte lacks [`OWNER_REFRESH_FLAG`],
    /// and otherwise converts the event's subject, kind and float
    /// words through the factory into the task slot (clearing the slot
    /// when no handler answers). Answers the conversion on the third
    /// path.
    pub fn refresh_flagged_owner(
        &mut self,
        owner_flags: u8,
        subject: Option<Handle32<EventSubject>>,
        kind: u32,
        weight: f32,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> FlaggedAnswer {
        let Some(owner) = self.owner else {
            return FlaggedAnswer::NoOwner;
        };
        if owner_flags & OWNER_REFRESH_FLAG == 0 {
            return FlaggedAnswer::KeepOwner(owner);
        };
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return FlaggedAnswer::Converted(None);
        };
        let answer = factory.convert(
            handle,
            ConvertRequest::SubjectKindFloat {
                subject,
                kind,
                weight_bits: weight.to_bits(),
            },
        );
        self.pending = answer;
        FlaggedAnswer::Converted(answer)
    }

    /// Response-build slot: builds a fresh response through the factory.
    ///
    /// Restates the verified slot that ignores its arguments, asks the
    /// shared manager for a fresh object and builds it into the task
    /// slot (storing zero when the allocation answers null). Answers
    /// the stored word either way. The allocator/builder pair shares
    /// the [`TaskFactory`] trait: the lookup allocates, the conversion
    /// builds.
    pub fn build_response(
        &mut self,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let answer = factory.convert(handle, ConvertRequest::Build);
        self.pending = answer;
        answer
    }

    /// Route slot: polls the owner, then clears, builds or dispatches.
    ///
    /// Restates the verified slot that polls the event's owner twice
    /// and stops with no store when the first answer is null or the
    /// second equals this handler's owner; otherwise the kind decides:
    /// [`KIND_CLEAR`] and [`KIND_RESET_B`] clear the task slot,
    /// [`KIND_ROUTE_BUILD`] builds through the factory from the event's
    /// block and two float words into the task slot (clearing it when
    /// no handler answers), and any other kind dispatches to the
    /// handler's own block slot. The original answers a constant zero,
    /// which carries no meaning.
    pub fn route_by_owner_and_kind(
        &mut self,
        input: RouteInput,
        source: &mut impl EventSource,
        dispatch: &mut impl EventDispatch,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) {
        if source.poll_owner().is_none() {
            return;
        }
        if source.poll_owner() == self.owner {
            return;
        }
        if input.kind == KIND_CLEAR || input.kind == KIND_RESET_B {
            self.pending = None;
            return;
        }
        if input.kind != KIND_ROUTE_BUILD {
            dispatch.dispatch_block(input.kind, input.event, ROUTE_BLOCK);
            return;
        }
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return;
        };
        let answer = factory.convert(
            handle,
            ConvertRequest::BlockFloats {
                block_offset: ROUTE_BLOCK,
                first_bits: input.first.to_bits(),
                second_bits: input.second.to_bits(),
            },
        );
        self.pending = answer;
    }

    /// Dual-path float slot: evaluates a scalar, converts through one
    /// of two sibling converters.
    ///
    /// Restates the verified slot that looks up a conversion handler
    /// from the shared manager (clearing the task slot and stopping
    /// when none answers), evaluates a scalar from the event's words
    /// through the scalar call, and converts the scalar with the
    /// event's vector block through one of two sibling converters
    /// picked by bit 1 of the selector byte, storing the answer in the
    /// task slot. Answers the conversion either way.
    pub fn answer_scalar_block(
        &mut self,
        selector: u8,
        event: Handle32<EventRef>,
        words: BlockWords,
        scalar: &mut impl ScalarEval,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let value = scalar.eval_block(event, words);
        let second = selector & BLOCK_SELECT_BIT == 0;
        let answer = factory.convert(
            handle,
            ConvertRequest::ScalarBlock {
                scalar_bits: value.to_bits(),
                words,
                second,
            },
        );
        self.pending = answer;
        answer
    }

    /// Routed-probe slot: two kind probes gate a lookup and conversion.
    ///
    /// Restates the verified slot that probes the event's kind and
    /// stops with the probe's answer on kind [`PROBE_EARLY_KIND`] with
    /// a null secondary link, or on kind [`PROBE_LATE_KIND`] when the
    /// readiness check's low byte is set; otherwise the kind word
    /// decides: [`KIND_CLEAR`] stores null, [`KIND_ROUTED_CONVERT`]
    /// runs a two-stage lookup (a zero low byte on the second stage
    /// stores null and answers it) and converts the link pair through
    /// the factory into the task slot (clearing it when no handler
    /// answers), and any other kind leaves the task slot alone and
    /// answers the kind minus the convert kind.
    pub fn answer_routed_probe(
        &mut self,
        links: EventLinks,
        kind: u32,
        event: &mut impl EventSource,
        staged: &mut impl StagedLookup,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> RoutedAnswer {
        if event.probe_kind() == PROBE_EARLY_KIND && links.secondary.is_none() {
            return RoutedAnswer::EarlyProbe(PROBE_EARLY_KIND);
        }
        if event.readiness() & 0xFF != 0 && event.probe_kind() == PROBE_LATE_KIND {
            return RoutedAnswer::EarlyProbe(PROBE_LATE_KIND);
        }
        if kind == KIND_CLEAR {
            self.pending = None;
            return RoutedAnswer::StoredNull;
        }
        if kind != KIND_ROUTED_CONVERT {
            return RoutedAnswer::Unrouted(kind.wrapping_sub(KIND_ROUTED_CONVERT));
        }
        let mid = staged.stage(links.primary, links.secondary);
        let fin = staged.finish(mid, links.primary, links.secondary);
        if fin & 0xFF == 0 {
            self.pending = None;
            return RoutedAnswer::LookupFailed(fin);
        }
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return RoutedAnswer::Converted(None);
        };
        let answer = factory.convert(
            handle,
            ConvertRequest::Pair {
                primary: links.primary,
                secondary: links.secondary,
            },
        );
        self.pending = answer;
        RoutedAnswer::Converted(answer)
    }

    /// Guarded-probe slot: checks a registry, gates on kind and marker
    /// bits, converts the probe.
    ///
    /// Restates the verified slot that probes the event, checks the
    /// kind against the registry (a nonzero answer ends the call and
    /// is returned), and requires kind [`KIND_GUARDED_CONVERT`], a live
    /// probe and marker bits [`MARKER_WANT`] in the probe's status word
    /// before converting the probe through the factory into the task
    /// slot (clearing it when no handler answers). A registry release
    /// runs on every converted probe, and its answer is the slot's
    /// answer. The owner word travels as a parameter rather than from
    /// this handler because the slot reads it but faults unless it is
    /// live, which the non-optional handle states in the type.
    pub fn answer_guarded_probe(
        &mut self,
        owner: Handle32<Owner>,
        probe: ProbeInput,
        event: &mut impl EventSource,
        registry: &mut impl ProbeRegistry,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> GuardedProbeAnswer {
        let found = event.probe();
        let checked = registry.check(owner, probe.kind);
        if checked != 0 {
            return GuardedProbeAnswer::Registry(checked);
        }
        if probe.kind != KIND_GUARDED_CONVERT {
            return GuardedProbeAnswer::Rejected(0);
        }
        let Some(found) = found else {
            return GuardedProbeAnswer::Rejected(0);
        };
        let bits = probe.status & MARKER_MASK;
        if bits != MARKER_WANT {
            return GuardedProbeAnswer::Rejected(bits);
        }
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            let released = registry.release(owner, found);
            return GuardedProbeAnswer::Converted {
                task: None,
                released,
            };
        };
        let answer = factory.convert(handle, ConvertRequest::Probe(found));
        self.pending = answer;
        let released = registry.release(owner, found);
        GuardedProbeAnswer::Converted {
            task: answer,
            released,
        }
    }

    /// Scalar-vector slot: evaluates a scalar, converts it with a vector.
    ///
    /// Restates the verified slot that picks one of two event-relative
    /// vector bases by the flag word, evaluates a scalar from the input
    /// word, looks up a conversion handler from the shared manager
    /// (clearing the task slot and stopping when none answers) and
    /// converts the scalar with the vector base and the float word into
    /// the task slot. Answers the conversion either way.
    pub fn answer_scalar_vec(
        &mut self,
        input: VecInput,
        event: Handle32<EventRef>,
        scalar: &mut impl ScalarEval,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> Option<Handle32<Task>> {
        let value = scalar.eval_word(input.scalar_input);
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return None;
        };
        let base = if input.flag == VEC_FIRST_FLAG {
            VEC_FIRST_BASE
        } else {
            VEC_SECOND_BASE
        };
        let answer = factory.convert(
            handle,
            ConvertRequest::ScalarVec {
                scalar_bits: value.to_bits(),
                vec_offset: base,
                weight_bits: input.weight.to_bits(),
            },
        );
        self.pending = answer;
        answer
    }

    /// Settle refresh slot: keeps the task unless the owner is settled.
    ///
    /// Restates the verified slot that asks the owner's ready slot and,
    /// when ready and the follow-up check's low byte is set, keeps its
    /// task and answers the check; otherwise it converts the settle
    /// request through the factory into the task slot (clearing it when
    /// no handler answers). The owner word travels as a parameter
    /// rather than from this handler because the slot probes it but
    /// faults unless it is live, which the non-optional handle states
    /// in the type.
    pub fn refresh_unless_settled(
        &mut self,
        owner: Handle32<Owner>,
        settle: &mut impl SettleStatus,
        factory: &mut impl TaskFactory,
        state: &FactoryState,
    ) -> SettledAnswer {
        if settle.owner_ready(owner) & 0xFF != 0 {
            let check = settle.confirm_settled();
            if check & 0xFF != 0 {
                return SettledAnswer::Settled(check);
            }
        }
        let Some(handle) = factory.lookup(state.manager()) else {
            self.pending = None;
            return SettledAnswer::Refreshed(None);
        };
        let answer = factory.convert(handle, ConvertRequest::Code(SETTLE_BUILD_KIND));
        self.pending = answer;
        SettledAnswer::Refreshed(answer)
    }
}

/// How the child slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryAnswer {
    /// The child word was null: the answer is the event itself and the
    /// task slot is untouched.
    Passthrough(Handle32<EventRef>),
    /// The child was converted: the answer is the conversion, also
    /// stored in the task slot.
    Converted(Option<Handle32<Task>>),
}

/// How the owner-gated refresh slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardedAnswer {
    /// The owner's flag byte carried the keep bit: the task slot is
    /// untouched and the answer is the owner.
    KeepOwner(Handle32<Owner>),
    /// The subject word was null: the task slot is untouched and the
    /// answer is the event.
    KeepEvent(Handle32<EventRef>),
    /// The subject was converted: the answer is the conversion, also
    /// stored in the task slot.
    Converted(Option<Handle32<Task>>),
}

/// How the flagged-owner refresh slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlaggedAnswer {
    /// The owner word was null: the task slot is untouched. The
    /// original answers zero, which carries no meaning and is not
    /// modelled.
    NoOwner,
    /// The owner's flag byte lacked the refresh bit: the task slot is
    /// untouched and the answer is the owner.
    KeepOwner(Handle32<Owner>),
    /// The event converted: the answer is the conversion, also stored
    /// in the task slot.
    Converted(Option<Handle32<Task>>),
}

/// How the type-gated slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatedAnswer {
    /// The kind was [`KIND_CLEAR`](crate::event_handler::KIND_CLEAR):
    /// the task slot was cleared and the answer is the kind word.
    Cleared,
    /// Any other non-converted kind: the task slot is untouched and
    /// the answer is the kind word.
    Ignored,
    /// The convert kind: the answer is the conversion, also stored in
    /// the task slot.
    Converted(Option<Handle32<Task>>),
}

/// The shared task factory behind the manager word (opaque identity).
///
/// Opaque: the lifted handler carries it into the factory lookup, never
/// interprets it.
#[derive(Debug)]
pub struct TaskManager;

/// A conversion handler answered by the factory lookup (opaque identity).
#[derive(Debug)]
pub struct FactoryHandle;

/// Shared factory state: the manager word the factory-pair slots read.
///
/// The original keeps this word in a global; the lift carries it as an
/// explicit argument, so tests build only what is involved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FactoryState {
    /// The manager word: the factory lookup's argument.
    manager: Option<Handle32<TaskManager>>,
}

impl FactoryState {
    /// Builds the state from the manager word.
    #[must_use]
    pub const fn new(manager: Option<Handle32<TaskManager>>) -> Self {
        Self { manager }
    }

    /// The manager word.
    #[must_use]
    pub const fn manager(self) -> Option<Handle32<TaskManager>> {
        self.manager
    }
}

/// One conversion request through the factory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertRequest {
    /// Convert a fixed request code.
    Code(u32),
    /// Convert with no request word (the gated slot's conversion).
    Plain,
    /// Convert an event's child word.
    Child(Handle32<EventChild>),
    /// Convert an id with a staged word (the tagged slot's conversion).
    /// The staged word is the event identity with its low byte replaced
    /// by the tag; the target call carries a zero pad word after it.
    Tagged {
        /// The event id word.
        id: u32,
        /// The staged word: identity bits plus tag byte.
        staged: u32,
    },
    /// Convert an event's subject and kind words (the mark-seen slot).
    /// The subject is carried through even when null.
    SubjectKind {
        /// The event's subject word.
        subject: Option<Handle32<EventSubject>>,
        /// The event's kind word.
        kind: u32,
    },
    /// Convert an event's subject word (the subject refresh slots).
    Subject(Handle32<EventSubject>),
    /// Convert subject, kind and the float word, bitwise (the flagged slot).
    SubjectKindFloat {
        /// The event's subject word (carried through even when null).
        subject: Option<Handle32<EventSubject>>,
        /// The event's kind word.
        kind: u32,
        /// The event's float word as bits; no arithmetic touches it.
        weight_bits: u32,
    },
    /// Build a fresh response object (the build slot). The target call
    /// carries a fixed all-ones word after the handle, pinned by the
    /// proof rather than modelled here.
    Build,
    /// Build from an event block and two float words, bitwise (the
    /// route slot). The block travels as its byte offset from the
    /// event; the target call carries the block address first.
    BlockFloats {
        /// Byte offset of the event block from the event.
        block_offset: u32,
        /// The first float word as bits; no arithmetic touches it.
        first_bits: u32,
        /// The second float word as bits; no arithmetic touches it.
        second_bits: u32,
    },
    /// Convert a scalar with the event's vector block (the dual-path
    /// slot). The block words travel as values; the target call
    /// carries the event-relative addresses the proof maps.
    ScalarBlock {
        /// The evaluated scalar as bits; no arithmetic touches it.
        scalar_bits: u32,
        /// The block words the scalar call read.
        words: BlockWords,
        /// Which sibling converter runs: set for the second, clear for
        /// the first.
        second: bool,
    },
    /// Convert a link pair (the routed-probe slot's conversion).
    Pair {
        /// The primary link word (carried through even when null).
        primary: Option<Handle32<EventLink>>,
        /// The secondary link word (carried through even when null).
        secondary: Option<Handle32<EventLink>>,
    },
    /// Convert an event's probe (the guarded-probe slot's conversion).
    /// The target call carries a trailing zero word after it, pinned
    /// by the proof rather than modelled here.
    Probe(Handle32<EventProbe>),
    /// Convert a scalar with a vector base (the scalar-vector slot).
    /// The base travels as its byte offset from the event; the target
    /// call carries the event-relative address the proof maps, then a
    /// trailing zero word pinned by the proof.
    ScalarVec {
        /// The evaluated scalar as bits; no arithmetic touches it.
        scalar_bits: u32,
        /// Byte offset of the vector base from the event.
        vec_offset: u32,
        /// The float word as bits; no arithmetic touches it.
        weight_bits: u32,
    },
}

/// What the handler asks of the task factory: the factory-side
/// collaborator.
///
/// The factory-pair slots share one shape: a lookup call against the
/// shared manager, then, when a handler answers, one conversion call.
/// Production code implements this on the lifted factory; tests pass a
/// fake that records calls and scripts answers.
pub trait TaskFactory {
    /// Looks up a conversion handler from the shared manager.
    fn lookup(&mut self, manager: Option<Handle32<TaskManager>>)
    -> Option<Handle32<FactoryHandle>>;

    /// Converts one request through an answering handler.
    fn convert(
        &mut self,
        handle: Handle32<FactoryHandle>,
        request: ConvertRequest,
    ) -> Option<Handle32<Task>>;
}

/// The event as the route slot sees it: its kind, its identity and the
/// two float words the build call carries.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteInput {
    /// The event's kind word.
    pub kind: u32,
    /// The event's identity (its address on the 32-bit side).
    pub event: Handle32<EventRef>,
    /// The build call's first float word.
    pub first: f32,
    /// The build call's second float word.
    pub second: f32,
}

/// The event words the dual-path slot threads through its scalar and
/// conversion calls, named by byte offset from the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockWords {
    /// The word at event +0x34: the scalar call's input.
    pub w34: u32,
    /// The word at event +0x30.
    pub w30: u32,
    /// The word at event +0x3C.
    pub w3c: u32,
    /// The word at event +0x40.
    pub w40: u32,
}

/// The link pair the routed-probe slot reads from the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventLinks {
    /// The primary link word.
    pub primary: Option<Handle32<EventLink>>,
    /// The secondary link word (its nullness gates the early probe).
    pub secondary: Option<Handle32<EventLink>>,
}

/// The event as the guarded-probe slot sees it: its kind and its
/// probe's status word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbeInput {
    /// The event's kind word.
    pub kind: u32,
    /// The probe's status word (only the marker bits are tested).
    pub status: u32,
}

/// The event as the scalar-vector slot sees it: its flag word, its
/// scalar input and its float word.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecInput {
    /// The flag word picking the vector base.
    pub flag: u32,
    /// The scalar call's input word.
    pub scalar_input: u32,
    /// The float word the conversion carries.
    pub weight: f32,
}

/// How the routed-probe slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutedAnswer {
    /// A kind probe ended the call: the task slot is untouched and the
    /// answer is the probe's kind.
    EarlyProbe(u32),
    /// The kind clears: null was stored and zero answered.
    StoredNull,
    /// Any other non-converted kind: the task slot is untouched and
    /// the answer is the kind minus the convert kind.
    Unrouted(u32),
    /// The lookup's second stage failed its low-byte test: null was
    /// stored and the stage's answer returned.
    LookupFailed(u32),
    /// The link pair converted: the answer is the conversion, also
    /// stored in the task slot.
    Converted(Option<Handle32<Task>>),
}

/// How the guarded-probe slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardedProbeAnswer {
    /// The registry check answered nonzero: nothing is stored and the
    /// answer is the check's.
    Registry(u32),
    /// A gate failed (kind, probe or marker bits): nothing is stored
    /// and the answer is the gate's word.
    Rejected(u32),
    /// The probe converted: the task slot holds the conversion and
    /// the answer is the registry release's.
    Converted {
        /// The conversion, also stored in the task slot.
        task: Option<Handle32<Task>>,
        /// The registry release's answer: the slot's answer.
        released: u32,
    },
}

/// How the settle slot answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettledAnswer {
    /// The owner is settled: the task slot is untouched and the answer
    /// is the follow-up check's.
    Settled(u32),
    /// The settle request converted: the answer is the conversion,
    /// also stored in the task slot.
    Refreshed(Option<Handle32<Task>>),
}
