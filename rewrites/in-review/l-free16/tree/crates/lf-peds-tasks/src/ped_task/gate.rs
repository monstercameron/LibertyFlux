//! The task request gate: whether a ped task request may proceed.
//!
//! [`admits`] decides from four inputs. The task's vtable flag word and two
//! global words are read by the caller (they live outside the lifted
//! data); the status probe is a callee on the task; the candidate's two
//! words (state and sub-flag) are read by the caller. The request's flags
//! and mode arrive as plain values.
//!
//! The gate is closed when the vtable flag's bit is clear and either the
//! mode is nonzero or the first global is nonzero, and then the status
//! probe must answer nonzero and the high byte of the second global must be
//! zero. It is also closed by the reject bit of the flags. A candidate
//! whose state, masked, is not the busy value passes; a busy candidate
//! passes only when its sub-flag's bit four is clear.

#![forbid(unsafe_code)]

/// The vtable flag bit that, when set, skips the status probe.
pub const VT_FLAG_BIT: u32 = 0x0080_0000;
/// The request flag that rejects outright.
pub const REJECT_BIT: u32 = 0x1000;
/// The mask applied to the candidate's state word.
pub const STATE_MASK: u32 = 0x3c0;
/// The busy value of the masked state word.
pub const STATE_BUSY: u32 = 0xc0;
/// The bit of the candidate's sub-flag word that makes a busy candidate fail.
pub const SUB_BIT: u32 = 1 << 4;
/// The byte of the second global the gate reads, as a shift of its word.
pub const BYTE_SHIFT: u32 = 16;

/// The words the gate reads from outside the lifted data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateWords {
    /// The task's vtable flag word.
    pub vtable_flag: u32,
    /// The first global word (presence).
    pub presence: u32,
    /// The second global word; its byte at [`BYTE_SHIFT`] is read.
    pub busy: u32,
}

/// The candidate's two words, read by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateCandidate {
    /// The candidate's state word.
    pub state: u32,
    /// The candidate's sub-flag word.
    pub sub_flags: u32,
}

/// The status probe on the task: a callee the gate asks.
pub trait GateStatus {
    /// An opaque reference to the task.
    type Handle: Copy;

    /// Asks the task's status probe; zero means not ready.
    fn status(&mut self, task: Self::Handle) -> u8;
}

/// Whether a request for `task` with `flags` and `mode` may proceed against
/// `candidate`.
pub fn admits<C: GateStatus>(
    probe: &mut C,
    task: C::Handle,
    words: &GateWords,
    candidate: &GateCandidate,
    flags: u32,
    mode: u32,
) -> bool {
    let checked = words.vtable_flag & VT_FLAG_BIT == 0
        && ((mode as u8) != 0 || words.presence != 0);
    if checked {
        if probe.status(task) == 0 {
            return false;
        }
        if (words.busy >> BYTE_SHIFT) as u8 != 0 {
            return false;
        }
    }
    if flags & REJECT_BIT != 0 {
        return false;
    }
    if candidate.state & STATE_MASK != STATE_BUSY {
        return true;
    }
    candidate.sub_flags & SUB_BIT == 0
}
