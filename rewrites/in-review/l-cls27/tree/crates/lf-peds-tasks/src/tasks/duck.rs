//! The lifted duck: one owning type for `CTaskSimpleDuck`.
//!
//! The original keeps a ducking task per ped: a start tick and a span
//! that time it out, a signed level the update drains, done and event
//! flags, a marks word, and a tag byte the clone slot copies. This type
//! owns those words as ordinary Rust data and restates each verified
//! 32-bit method with behaviour in it as a method.
//!
//! Proof is differential: every lifted method runs against its verified
//! rewrite on the same generated inputs, comparing results and every
//! effect (see the `lf-taskdiff` test crate). Nothing here is verified
//! by the checker itself.
//!
//! What each lifted method covers, and what it narrows away from the
//! original, is recorded per method in [`registry`](crate::tasks::registry).

use lf_core::Handle32;

use crate::tasks::{TaskMgr, UninitTask};

/// The sink the duck's event slot announces to (opaque identity).
///
/// Opaque: the lifted task carries it into the announce call, never
/// interprets it. It becomes a real handle when its owner lifts.
#[derive(Debug)]
pub struct DuckSink;

/// The object the duck's event slot queries (opaque identity).
///
/// Opaque for the same reason as [`DuckSink`].
#[derive(Debug)]
pub struct DuckQuery;

/// The ped a duck task runs against (opaque identity).
///
/// Opaque for the same reason as [`DuckSink`].
#[derive(Debug)]
pub struct DuckPed;

/// The query answer the event slot reacts to.
pub const QUERY_CODE: u32 = 0x31;

/// The object state word the event slot reacts to.
pub const QUERY_STATE: u32 = 0x137;

/// The announce flag the update passes while sustaining the duck.
pub const SUSTAIN_FLAG: u32 = 1;

/// The announce flag the update passes on the finishing path.
pub const FINISH_FLAG: u32 = 0;

/// The three words the clone slot feeds the copy constructor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuckSpec {
    /// The tag byte.
    pub tag: u8,
    /// The timeout span.
    pub span: u32,
    /// The signed level, carried sign-extended.
    pub level: i16,
}

/// What the duck asks of the task pool: the allocator and the copy
/// constructor behind the clone slot.
pub trait DuckPool {
    /// Allocates a fresh block from the shared manager.
    fn alloc(&mut self, manager: Option<Handle32<TaskMgr>>) -> Option<Handle32<UninitTask>>;

    /// Copy-constructs a duck from the tag, span and level words.
    fn construct(
        &mut self,
        block: Handle32<UninitTask>,
        spec: DuckSpec,
    ) -> Option<Handle32<DuckTask>>;
}

/// What the duck's event slot asks of its collaborators: the announce
/// sink and the queried object.
pub trait DuckEvent {
    /// Announces the duck to the sink.
    fn announce(&mut self, sink: Option<Handle32<DuckSink>>);

    /// Queries the object (answers [`QUERY_CODE`] when it reacts).
    fn query(&mut self, obj: Handle32<DuckQuery>) -> u32;

    /// Triggers the object after a matching query and state.
    fn trigger(&mut self, obj: Handle32<DuckQuery>);
}

/// What the duck's update asks of the ped it runs against.
pub trait DuckPedSide {
    /// Samples the ped's motion blend (below the limit finishes the duck).
    fn sample(&mut self, ped: Handle32<DuckPed>) -> f32;

    /// Announces the duck to the ped with a flag word.
    fn announce(&mut self, ped: Handle32<DuckPed>, flag: u32);
}

/// What the duck's update asks of its own task side: the sustain helper,
/// the elapsed-time helper and its own finish check.
pub trait DuckTaskSide {
    /// Sustains the duck past a wrapped-around timeout.
    fn sustain(&mut self, ped: Handle32<DuckPed>);

    /// Answers the subtrahend the update drains the level with.
    fn elapsed(&mut self) -> u32;

    /// Runs the task's own finish check (its low byte decides).
    fn finish_check(&mut self, ped: Handle32<DuckPed>) -> u32;
}

/// A ducking task: its timeout, level, flags, marks and tag.
///
/// The 32-bit object holds these words past its header; the lifted form
/// owns them directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DuckTask {
    /// The marks word: bit 0 gates the finish check, bit 1 is set when
    /// the check passes.
    marks: u32,
    /// The start tick the timeout counts from.
    start: u32,
    /// The timeout span (zero disables the timeout arms).
    span: u32,
    /// The signed level: drained by the update, tested by the event slot.
    level: i16,
    /// The done flag: set once the timeout passes, forces finishing.
    done: bool,
    /// The event flag: set by unhandled events, skips the finish check.
    flagged: bool,
    /// The tag byte: copied into the copy constructor by the clone slot.
    tag: u8,
}

impl DuckTask {
    /// Builds a duck from its words.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        marks: u32,
        start: u32,
        span: u32,
        level: i16,
        done: bool,
        flagged: bool,
        tag: u8,
    ) -> Self {
        Self {
            marks,
            start,
            span,
            level,
            done,
            flagged,
            tag,
        }
    }

    /// The marks word.
    #[must_use]
    pub const fn marks(self) -> u32 {
        self.marks
    }

    /// The start tick.
    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    /// The timeout span.
    #[must_use]
    pub const fn span(self) -> u32 {
        self.span
    }

    /// The signed level.
    #[must_use]
    pub const fn level(self) -> i16 {
        self.level
    }

    /// The done flag.
    #[must_use]
    pub const fn done(self) -> bool {
        self.done
    }

    /// The event flag.
    #[must_use]
    pub const fn flagged(self) -> bool {
        self.flagged
    }

    /// The tag byte.
    #[must_use]
    pub const fn tag(self) -> u8 {
        self.tag
    }

    /// Clone slot: allocates a fresh object and constructs a copy.
    ///
    /// Restates the verified slot that asks the pool for a block from
    /// the shared manager and, when one answers, copy-constructs it
    /// from the tag, span and sign-extended level words, answering the
    /// construction. Answers null when the allocation fails, without
    /// constructing. The source object is never modified.
    pub fn clone_task(
        &self,
        manager: Option<Handle32<TaskMgr>>,
        pool: &mut impl DuckPool,
    ) -> Option<Handle32<DuckTask>> {
        let block = pool.alloc(manager)?;
        pool.construct(
            block,
            DuckSpec {
                tag: self.tag,
                span: self.span,
                level: self.level,
            },
        )
    }

    /// Event slot: acknowledges duck events, flags anything else.
    ///
    /// Restates the verified slot that announces to the sink and answers
    /// 1 on event 2; on event 1, when the level is above -1 and an object
    /// is given, queries it and triggers it on a [`QUERY_CODE`] answer
    /// with a [`QUERY_STATE`] state word, then announces and answers 1;
    /// on any other event sets the event flag and answers 0.
    pub fn handle_event(
        &mut self,
        sink: Option<Handle32<DuckSink>>,
        event: u32,
        obj: Option<Handle32<DuckQuery>>,
        obj_state: u32,
        target: &mut impl DuckEvent,
    ) -> u32 {
        if event == 2 {
            target.announce(sink);
            return 1;
        }
        if event == 1 {
            if self.level > -1 {
                if let Some(obj) = obj {
                    if target.query(obj) == QUERY_CODE && obj_state == QUERY_STATE {
                        target.trigger(obj);
                    }
                }
            }
            target.announce(sink);
            return 1;
        }
        self.flagged = true;
        0
    }

    /// Update slot: times out the duck, then finishes or sustains it.
    ///
    /// Restates the verified slot. When the span is set and the tick
    /// minus the start reaches it (unsigned), the done flag is set. A
    /// set done flag takes the finishing path; otherwise the ped is
    /// sampled and a sample below the limit finishes too. Sustaining
    /// announces [`SUSTAIN_FLAG`], sustains past a wrapped-around
    /// timeout, and drains the level by the elapsed helper, clamped at
    /// zero and stored back through 16 bits. Finishing runs the task's
    /// own check unless flagged or gated (setting marks bit 1 on a
    /// passing low byte), announces [`FINISH_FLAG`] and answers 1; the
    /// sustain paths answer 0.
    pub fn update(
        &mut self,
        ped: Handle32<DuckPed>,
        tick: u32,
        sample_limit: f32,
        peds: &mut impl DuckPedSide,
        parts: &mut impl DuckTaskSide,
    ) -> u32 {
        if self.span != 0 && tick.wrapping_sub(self.start) >= self.span {
            self.done = true;
        }
        if !self.done {
            let sample = peds.sample(ped);
            if !(sample_limit > sample) {
                peds.announce(ped, SUSTAIN_FLAG);
                if self.flagged {
                    return 0;
                }
                if self.span != 0 && tick > self.start.wrapping_add(self.span) {
                    parts.sustain(ped);
                }
                if self.level <= 0 {
                    return 0;
                }
                let sub = parts.elapsed();
                let left = self.span.wrapping_sub(sub);
                let clamped = if (left as i32) < 0 { 0 } else { left };
                self.level = clamped as u16 as i16;
                return 0;
            }
        }
        if !self.flagged && self.marks & 1 == 0 {
            if parts.finish_check(ped) & 0xFF != 0 {
                self.marks |= 2;
            }
        }
        peds.announce(ped, FINISH_FLAG);
        1
    }
}
