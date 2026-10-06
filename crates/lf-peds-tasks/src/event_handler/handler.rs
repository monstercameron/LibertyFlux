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

/// Owner flag bit that gates the owner-reading refresh slots.
///
/// The guarded slot keeps its task when the owner's flag byte carries
/// this bit; the flagged slot refreshes its task only then.
pub const OWNER_REFRESH_FLAG: u8 = 4;

/// What the handler calls back on itself: its own dispatch slot.
///
/// The forward-or-clear slot answers an event it does not clear by
/// calling this with the event's kind and payload words.
pub trait EventDispatch {
    /// Handles one forwarded event.
    fn dispatch_event(&mut self, kind: u32, payload: Option<Handle32<EventPayload>>);
}

/// What the handler calls on an event: the event-side collaborator.
///
/// The settle slot polls the event's owner until it settles; the adopt
/// slot asks the event to clone its task. Production code implements
/// this on the lifted event; tests implement it on fakes.
pub trait EventSource {
    /// Answers the event's owner word (the settle slot's poll).
    fn poll_owner(&mut self) -> Option<Handle32<Owner>>;

    /// Clones the event's task (the adopt slot's conversion).
    fn clone_task(&mut self) -> Option<Handle32<Task>>;
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
