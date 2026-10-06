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
    fn lookup(
        &mut self,
        manager: Option<Handle32<TaskManager>>,
    ) -> Option<Handle32<FactoryHandle>>;

    /// Converts one request through an answering handler.
    fn convert(
        &mut self,
        handle: Handle32<FactoryHandle>,
        request: ConvertRequest,
    ) -> Option<Handle32<Task>>;
}
