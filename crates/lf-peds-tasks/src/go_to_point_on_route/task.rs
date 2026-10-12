//! The go-to-point-on-route task object and its verified methods.
//!
//! The fields are the ones the verified methods read or write, named for
//! what they hold. Offsets are given in the doc comments as offsets from the
//! start of the task's object; they describe the layout the proof checks
//! against and are not used by the lifted code itself.

use core::hint::black_box;

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

/// The bit of the link's flag word that stops an advance: when it is set,
/// the progress is taken from the entry value unchanged.
const LINK_HOLD_BIT: u32 = 0x2;

/// An entry value at or below this bound is taken as the progress unchanged.
/// A NaN entry value is not at or below it, so it is advanced.
const ADVANCE_IDLE_BOUND: f32 = 1.0;
/// The entry value at which the high threshold takes over.
const ADVANCE_SPLIT: f32 = 2.0;
/// The threshold for an entry value at or below [`ADVANCE_SPLIT`].
const ADVANCE_LOW_THRESHOLD: f32 = 3.0;
/// The threshold for an entry value above [`ADVANCE_SPLIT`].
const ADVANCE_HIGH_THRESHOLD: f32 = 4.0;
/// The first blend weight, used by the low branch of the blend.
const ADVANCE_WEIGHT_A: f32 = 1.5;
/// The second blend weight, used by the high branch of the blend.
const ADVANCE_WEIGHT_B: f32 = 2.0;
/// The value the advance result is raised to when it is below it.
const ADVANCE_FLOOR: f32 = 1.1;
/// The double the scale helper is called with: one eighth of pi, as bits.
const ADVANCE_HELPER_ARG: u64 = 0x3FD9_21FB_6000_0000;

/// The fill bit of the first blend weight in the shared cache.
const CACHE_WEIGHT_A_FILLED: u32 = 0x1;
/// The fill bit of the second blend weight in the shared cache.
const CACHE_WEIGHT_B_FILLED: u32 = 0x2;
/// The fill bit of the scale in the shared cache.
const CACHE_SCALE_FILLED: u32 = 0x4;

/// The cache the advance method shares between its calls. Each cached word
/// is filled on first use: its fill bit in `flags` is clear until then, and
/// the first use stores the constant the method needs. The caller owns the
/// cache, so the lifted method holds no global state; a fresh cache is all
/// zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ProgressCache {
    /// The fill bits.
    pub flags: u32,
    /// The first blend weight, as the bits of a float.
    pub weight_a: u32,
    /// The second blend weight, as the bits of a float.
    pub weight_b: u32,
    /// The scale, as the bits of a float, filled from the scale helper.
    pub scale: u32,
}

/// The go-to-point-on-route task: the words its verified methods touch.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GoToPointOnRouteTask {
    /// The move speed, clamped to `[0, MAX_SPEED]` when set (offset `0x10`).
    pub speed: f32,
    /// The progress value the task reports (offset `0x18`).
    pub progress: f32,
    /// The value an advance starts from (offset `0x24`).
    pub entry_value: f32,
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
    /// The anchor point the advance measures from: x and y (offsets `0xD0`
    /// and `0xD4`).
    pub anchor: [f32; 2],
    /// The active byte; a zero byte makes the advance idle (offset `0xE0`).
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

    /// Advances the progress towards `target` along the ped's direction,
    /// and stores the result as the progress.
    ///
    /// The task is idle when its active byte is zero: it does nothing. Else
    /// the ped's link is asked for its hold bit; when it is set the progress
    /// becomes the entry value. An entry value at or below the idle bound
    /// does the same. Otherwise the blend is computed from the cache
    /// (filled on first use), the unit direction from the ped's position to
    /// `target` and from `target` to the anchor, and the clamped cosine
    /// between them, divided by the scale. A threshold test on the inverse
    /// length stores the entry value when it fails. The result is blended
    /// with the entry value, raised to the floor, and stored.
    ///
    /// `ped` is the handle the host reads the ped's link flags and position
    /// through; the host is asked for each only where the verified method
    /// reads it. `target` is passed by value.
    pub fn advance_progress<H: GoToPointOnRouteHost>(
        &mut self,
        host: &mut H,
        cache: &mut ProgressCache,
        ped: u32,
        target: [f32; 2],
    ) {
        if self.active == 0 {
            return;
        }
        let link_flags = host.ped_link_flags(ped);
        if (link_flags & LINK_HOLD_BIT) != 0 {
            self.progress = self.entry_value;
            return;
        }
        let entry = self.entry_value;
        if entry <= ADVANCE_IDLE_BOUND {
            self.progress = entry;
            return;
        }

        // The cached weights: each is filled from its constant on first use.
        let mut flags = cache.flags;
        let weight_a = if (flags & CACHE_WEIGHT_A_FILLED) != 0 {
            f32::from_bits(cache.weight_a)
        } else {
            flags |= CACHE_WEIGHT_A_FILLED;
            cache.flags = flags;
            cache.weight_a = ADVANCE_WEIGHT_A.to_bits();
            ADVANCE_WEIGHT_A
        };
        let weight_b = if (flags & CACHE_WEIGHT_B_FILLED) != 0 {
            f32::from_bits(cache.weight_b)
        } else {
            flags |= CACHE_WEIGHT_B_FILLED;
            cache.flags = flags;
            cache.weight_b = ADVANCE_WEIGHT_B.to_bits();
            ADVANCE_WEIGHT_B
        };
        // The threshold select: an entry value that is not above the split,
        // NaN included, takes the low arm.
        let (threshold, blend_b) = if !(entry > ADVANCE_SPLIT) {
            (ADVANCE_LOW_THRESHOLD, weight_a)
        } else {
            (ADVANCE_HIGH_THRESHOLD, weight_b)
        };

        // The unit direction from the ped's position to the target.
        let [ped_x, ped_y] = host.ped_position(ped);
        let dx = pinned_sub(target[0], ped_x);
        let dy = pinned_sub(target[1], ped_y);
        let d2 = pinned_add(pinned_mul(dy, dy), pinned_mul(dx, dx));
        let inv = reciprocal_length(d2);
        let nx = pinned_mul(inv, dx);
        let ny = pinned_mul(dy, inv);
        let nz = pinned_mul(inv, 0.0);
        let one_over = pinned_div(1.0, inv);

        // The scale: filled from the helper on first use.
        if (flags & CACHE_SCALE_FILLED) == 0 {
            let answer = host.double_helper(ADVANCE_HELPER_ARG);
            let scale = f64::from_bits(answer) as f32;
            flags |= CACHE_SCALE_FILLED;
            cache.flags = flags;
            cache.scale = scale.to_bits();
        }

        // A failed threshold test stores the entry value. An unordered
        // inverse length fails it too.
        if !(threshold > one_over) {
            self.progress = entry;
            return;
        }

        // The unit direction from the target to the anchor, and the clamped
        // cosine between the two directions over the scale.
        let ex = pinned_sub(self.anchor[0], target[0]);
        let ey = pinned_sub(self.anchor[1], target[1]);
        let e2 = pinned_add(pinned_mul(ey, ey), pinned_mul(ex, ex));
        let inv2 = reciprocal_length(e2);
        let scale = f32::from_bits(cache.scale);
        let mut t: f32 = if scale == 0.0 {
            1.0
        } else {
            let ux = pinned_mul(ex, inv2);
            let uy = pinned_mul(ey, inv2);
            let uz = pinned_mul(inv2, 0.0);
            let dot = pinned_add(
                pinned_add(pinned_mul(ux, nx), pinned_mul(uy, ny)),
                pinned_mul(uz, nz),
            );
            let q = pinned_div(dot, scale);
            if q < 0.0 {
                0.0
            } else if q > 1.0 {
                1.0
            } else {
                q
            }
        };

        // The blend with the entry value, then the two min-selects.
        let d = pinned_sub(entry, blend_b);
        t = pinned_mul(t, t);
        let e = pinned_sub(entry, d);
        t = pinned_add(pinned_mul(t, d), e);
        let capped = if t > entry { entry } else { t };
        self.progress = if ADVANCE_FLOOR > capped {
            ADVANCE_FLOOR
        } else {
            capped
        };
    }
}

/// Sub-float arithmetic is written through these helpers so that each float
/// operation keeps the operand order written here: `black_box` on both
/// operands stops the compiler from swapping them, which matters when both
/// are NaN (the result then carries the first operand's payload).
#[inline(always)]
fn pinned_sub(a: f32, b: f32) -> f32 {
    black_box(a) - black_box(b)
}

/// See [`pinned_sub`].
#[inline(always)]
fn pinned_add(a: f32, b: f32) -> f32 {
    black_box(a) + black_box(b)
}

/// See [`pinned_sub`].
#[inline(always)]
fn pinned_mul(a: f32, b: f32) -> f32 {
    black_box(a) * black_box(b)
}

/// See [`pinned_sub`].
#[inline(always)]
fn pinned_div(a: f32, b: f32) -> f32 {
    black_box(a) / black_box(b)
}

/// One over the square root of a squared length: zero for a zero length,
/// and NaN propagates.
#[inline(always)]
fn reciprocal_length(squared: f32) -> f32 {
    if squared == 0.0 {
        0.0
    } else {
        pinned_div(1.0, black_box(squared).sqrt())
    }
}
