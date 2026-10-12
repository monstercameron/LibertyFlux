//! The go-to-point-on-route task object and its verified methods.
//!
//! The fields are the ones the verified methods read or write, named for
//! what they hold. Offsets are given in the doc comments as offsets from the
//! start of the task's object; they describe the layout the proof checks
//! against and are not used by the lifted code itself.

use super::host::{CopyArgs, GoToPointOnRouteHost, TaskField};

/// The largest speed the task accepts; a larger request is stored as this.
pub const MAX_SPEED: f32 = 3.0;

/// The mode values that take the first return path. Any other mode takes
/// the second path, which sets [`UNRECOGNISED_MODE_FLAG`] in the flag word.
pub const MODE_ONE: u32 = 1;
/// The second of the two modes that take the first return path.
pub const MODE_TWO: u32 = 2;

/// The bit set in the flag word at offset `0xC4` for an unrecognised mode.
pub const UNRECOGNISED_MODE_FLAG: u32 = 0x10;

/// Mask that keeps the low byte of a mode word clear.
const MODE_UPPER_MASK: u32 = 0xFFFF_FF00;

/// The bits of the flag word at offset `0xB0` that a position change
/// clears before it sets the change bit.
const POSITION_FLAGS_KEEP: u32 = 0xFFFF_87FF;

/// The bit a position change sets in the flag word at offset `0xB0`.
const POSITION_CHANGED_BIT: u32 = 0x100;

/// The go-to-point-on-route task: the words its verified methods touch.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GoToPointOnRouteTask {
    /// The move speed, clamped to `[0, MAX_SPEED]` when set (offset `0x10`).
    pub speed: f32,
    /// The progress value the task reports (offset `0x18`).
    pub progress: f32,
    /// The position the task is moving to: x, y, z and one word the copy
    /// carries but no verified reader compares (offsets `0x2C` to `0x38`).
    pub position: [f32; 4],
    /// A single float the position code does not use; set through its own
    /// method (offset `0x8C`).
    pub value_8c: f32,
    /// A word the copy constructor receives (offset `0xA0`).
    pub word_a0: u32,
    /// The flag word the position change updates (offset `0xB0`).
    pub flags_b0: u32,
    /// The flag word the mode method updates; bits 0 and 1 are copied by
    /// the clone (offset `0xC4`).
    pub flags_c4: u32,
    /// The active byte; a zero byte makes the advance method idle
    /// (offset `0xE0`).
    pub active: u8,
}

impl GoToPointOnRouteTask {
    /// Whether the progress value is strictly above zero. A NaN progress
    /// is not positive.
    #[must_use]
    pub fn progress_is_positive(&self) -> bool {
        self.progress > 0.0
    }

    /// Stores a requested speed, clamped to `[0, MAX_SPEED]`. A value below
    /// zero is stored as zero and a value above the maximum as the maximum.
    /// A NaN request passes through unchanged: neither bound compares
    /// greater, so the request is stored as it is.
    pub fn set_speed(&mut self, requested: f32) {
        self.speed = if requested < 0.0 {
            0.0
        } else if requested > MAX_SPEED {
            MAX_SPEED
        } else {
            requested
        };
    }

    /// The first three words of the position: the point the task is moving
    /// to, as a caller copies it out. The fourth word is not part of it.
    #[must_use]
    pub fn position_point(&self) -> [f32; 3] {
        [self.position[0], self.position[1], self.position[2]]
    }

    /// Replaces the position with `source` when `force` asks for it or when
    /// any of the first three coordinates differs from the source's. Only
    /// the low byte of `force` is tested. Equal coordinates compare equal
    /// and an unordered (NaN) pair compares unequal, so a NaN coordinate
    /// always replaces the position. A replacement also clears the position
    /// change bits of the flag word at offset `0xB0` and sets bit `0x100`.
    pub fn set_position(&mut self, source: [f32; 4], force: u32) {
        let differs = self.position[0] != source[0]
            || self.position[1] != source[1]
            || self.position[2] != source[2];
        if (force as u8) != 0 || differs {
            self.position = source;
            self.flags_b0 = (self.flags_b0 & POSITION_FLAGS_KEEP) | POSITION_CHANGED_BIT;
        }
    }

    /// Stores `incoming` as the float at offset `0x8C` unless the stored
    /// value compares equal to it. Equal values, including a zero and a
    /// negative zero, leave the stored bits as they are; an unordered
    /// comparison counts as unequal and stores the new value.
    pub fn set_value_8c(&mut self, incoming: f32) {
        if self.value_8c != incoming {
            self.value_8c = incoming;
        }
    }

    /// Refreshes the route through the host, then answers for `mode`.
    ///
    /// The route refresh is called first, with the task and `requested`,
    /// and its answer is not used. The mode is then tested: `MODE_ONE` and
    /// `MODE_TWO` return `1`. Any other mode sets
    /// [`UNRECOGNISED_MODE_FLAG`] in the flag word, which the refresh may
    /// have changed, and returns the mode with its low byte cleared.
    pub fn refresh_mode<H: GoToPointOnRouteHost>(
        &mut self,
        host: &mut H,
        requested: u32,
        mode: u32,
    ) -> u32 {
        host.refresh_route(self, requested);
        if mode == MODE_ONE || mode == MODE_TWO {
            (mode & MODE_UPPER_MASK) | 1
        } else {
            self.flags_c4 |= UNRECOGNISED_MODE_FLAG;
            mode & MODE_UPPER_MASK
        }
    }

    /// Makes a copy of the task: allocates a new task through the host and,
    /// unless the allocation gives none, runs the class constructor on it
    /// with the words of this task. Returns the constructor's answer, or 0
    /// when the allocation gives no task (the constructor is not called).
    pub fn clone_task<H: GoToPointOnRouteHost>(&self, host: &mut H) -> u32 {
        let new_task = host.allocate_task();
        if new_task == 0 {
            return 0;
        }
        host.construct_copy(
            new_task,
            &CopyArgs {
                progress_bits: self.progress.to_bits(),
                block_40: TaskField(0x40),
                block_d0: TaskField(0xD0),
                active: u32::from(self.active),
                word_a0: self.word_a0,
                flag_bit_0: self.flags_c4 & 1,
                flag_bit_1: (self.flags_c4 >> 1) & 1,
            },
        )
    }
}
