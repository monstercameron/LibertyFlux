//! The shared ped-task state block: built from one object's floats, consumed vector by vector.
//!
//! The 32-bit form keeps the block in globals: two 3-vectors (the gate
//! vector and the work vector), three biases (gate, out, and the midpoint
//! enumeration's radius), a gate reference value, a gate upper bound and
//! one flag byte, plus the eight raw floats the block was built from. The
//! lift owns all of it as [`TaskStateBlock`]; two scratch words the
//! original wrote from uninitialised stack (pinned to zero by the rewrite)
//! are not modelled.
//!
//! Behaviour, two methods:
//!
//! - [`TaskStateBlock::build`]: from one task object's eight floats (a, c,
//!   f, raw4, b, e, d, raw7) copies the raws, stores the scaled differences
//!   as the gate vector (multiplier `0.0` when the sum of squared
//!   differences is exactly zero, `1/sqrt(S)` otherwise), the cross-like
//!   terms against zero as the work vector, the negated dots as the two
//!   biases, and the 2-D length as the range bound; clears the flag; issues
//!   the enumeration over the midpoints; copies the source word and stamps
//!   the object id and flag; answers whether the flag reads back clear.
//! - [`TaskStateBlock::consume`]: gates the target vector on the third
//!   component (`4.0` strictly above `|v2 - gate_ref|`) and on the gate dot
//!   landing in `[0, range_hi]` (either comparison unordered keeps it
//!   running), runs the vtable task call, the worker call and the check
//!   call, and raises the flag when the tail comparisons say so; always
//!   answers 1.

use lf_core::boundary::Handle32;

/// Tag for the opaque task-object identity.
#[derive(Debug)]
pub enum ObjTag {}

/// The task object as an opaque identity: carried across the calls that
/// take it, never interpreted.
pub type ObjHandle = Handle32<ObjTag>;

/// Gate limit on the third component's distance from the reference.
pub const GATE_LIMIT: f32 = 4.0;
/// Tail comparison bound.
pub const HALF: f32 = 0.5;
/// Tail comparison bound, negated.
pub const NEG_HALF: f32 = -0.5;
/// Object id the build stamps.
pub const OBJ_ID_VALUE: u32 = 2000;

/// The eight floats the build reads: (a, c, f, raw4, b, e, d, raw7).
#[derive(Clone, Copy, Debug)]
pub struct TaskFloats {
    /// First midpoint/difference pair, first member.
    pub a: f32,
    /// Second pair, first member.
    pub c: f32,
    /// Third pair, first member.
    pub f: f32,
    /// Copied to its raw slot, used nowhere else.
    pub raw4: f32,
    /// First pair, second member.
    pub b: f32,
    /// Second pair, second member.
    pub e: f32,
    /// Third pair, second member.
    pub d: f32,
    /// Copied to its raw slot, used nowhere else.
    pub raw7: f32,
}

/// The three words the build writes back to the task object.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TaskStamp {
    /// Copy of the source word.
    pub dword: u32,
    /// Stamped object id ([`OBJ_ID_VALUE`]).
    pub id: u32,
    /// Stamped flag (1).
    pub flag: u8,
}

/// The target vector the consume reads through the task object.
#[derive(Clone, Copy, Debug)]
pub struct TaskVec {
    /// Three floats of the vector record.
    pub v: [f32; 3],
}

/// The task object the consume takes: its opaque identity plus its vector.
#[derive(Clone, Copy, Debug)]
pub struct TaskTarget {
    /// Opaque identity, forwarded to the vtable and worker calls.
    pub obj: ObjHandle,
    /// The vector record behind it.
    pub vec: TaskVec,
}

/// The shared state block.
#[derive(Clone, Debug)]
pub struct TaskStateBlock {
    /// Flag byte: cleared by the build, maybe raised by the consume.
    pub flag: bool,
    /// Negated dot of (a, c, f) with the cross-like terms.
    pub out_bias: f32,
    /// Negated dot of (a, c, f) with the scaled differences.
    pub gate_bias: f32,
    /// 2-D length over (e-c, b-a): upper bound of the gate-dot window.
    pub range_hi: f32,
    /// Gate vector: the scaled differences.
    pub gate: [f32; 3],
    /// Work vector: the cross-like terms against zero.
    pub work: [f32; 3],
    /// Raw copies in object order: (a, c, f, raw4, b, e, d, raw7).
    pub raw: [f32; 8],
    /// Word the build copies into the object (another subsystem's global).
    pub copy_source: u32,
}

impl TaskStateBlock {
    /// The gate reference: the third raw float.
    #[must_use]
    pub fn gate_ref(&self) -> f32 {
        self.raw[2]
    }
}

/// The midpoint enumeration (one numbered callee).
///
/// The enumerator receives the state because it may re-enter it: the
/// build answers whether the flag reads back clear afterwards.
pub trait Enumerate {
    /// Enumerates the midpoints, possibly touching the state.
    fn enumerate(&mut self, state: &mut TaskStateBlock, mid: [f32; 3]);
}

/// The vtable task call behind the object's slot (a direct call, not a
/// numbered slot): answers five words of which the first three are read.
pub trait VTask {
    /// Runs the call for `obj`, answering the three read words.
    fn run(&mut self, obj: ObjHandle) -> [f32; 3];
}

/// The nine-word worker call (one numbered callee): only the v2 word
/// carries meaning; the scratch address, the fixed 0.25 word and the six
/// zeros are pinned by the proof.
pub trait Worker {
    /// Runs the call for `obj` with the third component.
    fn run(&mut self, obj: ObjHandle, v2: f32);
}

/// The two-address check call (one numbered callee): both addresses are
/// fixed state offsets, so only the answer travels.
pub trait Check {
    /// Runs the check; the low byte of the answer steers the tail.
    fn check(&mut self) -> u32;
}

impl<F: FnMut(&mut TaskStateBlock, [f32; 3])> Enumerate for F {
    fn enumerate(&mut self, state: &mut TaskStateBlock, mid: [f32; 3]) {
        self(state, mid);
    }
}

impl<F: FnMut(ObjHandle) -> [f32; 3]> VTask for F {
    fn run(&mut self, obj: ObjHandle) -> [f32; 3] {
        self(obj)
    }
}

impl<F: FnMut(ObjHandle, f32)> Worker for F {
    fn run(&mut self, obj: ObjHandle, v2: f32) {
        self(obj, v2);
    }
}

impl<F: FnMut() -> u32> Check for F {
    fn check(&mut self) -> u32 {
        self()
    }
}

#[inline(always)]
fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn neg(a: f32) -> f32 {
    f32::from_bits(a.to_bits() ^ 0x8000_0000)
}

impl TaskStateBlock {
    /// Builds the block from the object's eight floats and stamps the object.
    ///
    /// Answers whether the flag reads back clear after the enumeration
    /// (false only when the enumerator re-entered the state).
    pub fn build<E: Enumerate>(
        state: &mut TaskStateBlock,
        floats: &TaskFloats,
        stamp: &mut TaskStamp,
        enumerator: &mut E,
    ) -> bool {
        let (a, c, f) = (floats.a, floats.c, floats.f);
        let (b, e, d) = (floats.b, floats.e, floats.d);
        let m0 = mul(add(a, b), 0.5);
        let m1 = mul(add(c, e), 0.5);
        let m2 = mul(add(f, d), 0.5);
        let d0 = sub(b, a);
        let d1 = sub(e, c);
        let d2 = sub(d, f);
        // Exactly `== 0.0`: NaN takes the division path, as the original's
        // unordered-aware test does.
        let sumsq = add(add(mul(d1, d1), mul(d0, d0)), mul(d2, d2));
        let mult = if sumsq == 0.0 {
            0.0
        } else {
            core::hint::black_box(1.0f32) / core::hint::black_box(sumsq.sqrt())
        };
        let s0 = mul(d0, mult);
        let s1 = mul(d1, mult);
        let s2 = mul(d2, mult);
        // Cross-like terms against zero, kept in this operand order:
        // a zero factor still signs and NaNs its product.
        let t2 = sub(mul(s0, 0.0), mul(s1, 0.0));
        let t0 = sub(s1, mul(s2, 0.0));
        let t1 = sub(mul(s2, 0.0), s0);
        state.raw = [a, c, f, floats.raw4, b, e, d, floats.raw7];
        state.gate = [s0, s1, s2];
        let gdot = add(add(mul(c, s1), mul(a, s0)), mul(f, s2));
        state.gate_bias = neg(gdot);
        state.work = [t0, t1, t2];
        let len = add(mul(sub(e, c), sub(e, c)), mul(sub(b, a), sub(b, a))).sqrt();
        state.range_hi = len;
        state.flag = false;
        let odot = add(add(mul(c, t1), mul(a, t0)), mul(f, t2));
        state.out_bias = neg(odot);
        enumerator.enumerate(state, [m0, m1, m2]);
        stamp.dword = state.copy_source;
        stamp.id = OBJ_ID_VALUE;
        stamp.flag = 1;
        !state.flag
    }

    /// Consumes the target vector, maybe raising the flag. Always answers 1.
    ///
    /// The vtable, worker and check collaborators run only past both gates.
    pub fn consume<V: VTask, W: Worker, C: Check>(
        &mut self,
        target: &TaskTarget,
        vtask: &mut V,
        worker: &mut W,
        check: &mut C,
    ) -> u8 {
        let [v0, v1, v2] = target.vec.v;
        // Gate 1: 4.0 strictly above the absolute gap.
        let gap = f32::from_bits(sub(v2, self.gate_ref()).to_bits() & 0x7FFF_FFFF);
        if GATE_LIMIT > gap {
            // Gate 2: the gate dot lands in [0, range_hi]; an unordered
            // comparison on either side keeps it running.
            let gate = add(
                add(add(mul(self.gate[1], v1), mul(self.gate[0], v0)), mul(self.gate[2], v2)),
                self.gate_bias,
            );
            if !(0.0 > gate) && !(gate > self.range_hi) {
                let rout = add(
                    add(
                        add(mul(self.work[1], v1), mul(self.work[0], v0)),
                        mul(self.work[2], v2),
                    ),
                    self.out_bias,
                );
                let o = vtask.run(target.obj);
                let rwork = add(add(mul(self.work[1], o[1]), mul(self.work[0], o[0])), mul(self.work[2], o[2]));
                worker.run(target.obj, v2);
                let ok = check.check() & 0xff;
                let mut raise = false;
                if ok == 0 && HALF > rwork {
                    raise = true;
                }
                if !raise {
                    if rout > 0.0 {
                        if NEG_HALF > rwork {
                            raise = true;
                        }
                    }
                    if !raise && 0.0 > rout && rwork > HALF {
                        raise = true;
                    }
                }
                if raise {
                    self.flag = true;
                }
            }
        }
        1
    }
}
