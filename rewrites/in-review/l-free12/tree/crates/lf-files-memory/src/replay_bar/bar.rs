//! The replay bar control: time range, stamped slots, selection, hit rects.
//!
//! Lifted from the fifteen verified rewrites that take the bar object as
//! `this`: the two index mappers, the slot search, the millisecond
//! conversion, the two hit tests, the cursor store, the scaled position,
//! the two scored scans, the two ratio publishers, the bound clamp, the
//! entry selection and the time factor. The 32-bit object spans about 260
//! bytes; the lift owns every word some routine reads or writes.

use super::clock::{BaseSelect, ClockHandle, ClockRead, TimeBases};

/// Publish code the selection sends when the stamp exceeds the first sample.
pub const PUBLISH_ABOVE: u32 = 6;
/// Publish code the selection sends when the stamp is below the second sample.
pub const PUBLISH_BELOW: u32 = 0x0e;
/// Marker the selection always writes beside the selected index.
pub const SEL_MARK: u32 = 0xffff_ffff;
/// Default tag word the scored scans publish before scoring.
pub const SCAN_DEFAULT_TAG: u32 = 100;
/// Base the time factor's tick reading is subtracted from.
pub const TICKS_BASE: f32 = 17.0;

/// One bar slot: a tag byte and a stamp word.
///
/// The 32-bit entries are larger objects; the group reads only the tag at
/// entry `+0x01` and the stamp at `+0x14`, so those are what the lift owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplaySlot {
    /// The tag byte.
    pub tag: u8,
    /// The stamp word.
    pub stamp: u32,
}

/// The two words the entry selection publishes through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StampPublish {
    /// Flag word: the selection sets bit 0 on every publish.
    pub flags: u32,
    /// The last published stamp.
    pub stamp: u32,
}

/// Samples a word with no arguments: the no-arg cdecl sampler.
///
/// One trait serves the slot-ratio reading, the bound-clamp sample, the
/// entry selection's two stamp samples and the notifier's probe: the
/// 32-bit side is the same slot shape in each, and each method documents
/// what its samples mean.
pub trait Sample {
    /// Takes one sample.
    fn sample(&mut self) -> u32;
}

impl<F: FnMut() -> u32> Sample for F {
    fn sample(&mut self) -> u32 {
        self()
    }
}

/// Scores a stamp word: the scored scans' and the slot measure's callee.
///
/// The 32-bit callee also takes the bar pointer and a trailing zero; both
/// are fixed per call (the proof pins the zero), so the lift takes the
/// stamp only.
pub trait ScoreStamp {
    /// Scores `stamp`.
    fn score(&mut self, stamp: u32) -> u32;
}

impl<F: FnMut(u32) -> u32> ScoreStamp for F {
    fn score(&mut self, stamp: u32) -> u32 {
        self(stamp)
    }
}

/// Runs when the entry selection changes to a new index.
pub trait SelectionWatch {
    /// Notes that the selection moved.
    fn selection_changed(&mut self);
}

impl<F: FnMut()> SelectionWatch for F {
    fn selection_changed(&mut self) {
        self();
    }
}

/// Publishes a selection event code (6 or `0x0e`: [`PUBLISH_ABOVE`], [`PUBLISH_BELOW`]).
pub trait Publish {
    /// Publishes `code`.
    fn publish(&mut self, code: u32);
}

impl<F: FnMut(u32)> Publish for F {
    fn publish(&mut self, code: u32) {
        self(code);
    }
}

/// The replay bar control object.
#[derive(Debug, Clone, PartialEq)]
pub struct ReplayBar {
    /// Seconds field: the millisecond conversion's source.
    pub seconds: f32,
    /// Bar origin: the scaled position's added float.
    pub origin: f32,
    /// Bar weight: the time factor's multiplier.
    pub weight: f32,
    /// Bar width: the scaled position's multiplier.
    pub width: f32,
    /// Cursor word, stored as bits (no routine in the group reads it).
    pub cursor: u32,
    /// Clock link: `None` is the null link, which answers `+0.0`.
    pub clock: Option<ClockHandle>,
    /// The slot entries in table order.
    ///
    /// At most 65,535 entries: the 32-bit table's count word is 16 bits,
    /// and the proof plants exactly this length there.
    pub slots: Vec<ReplaySlot>,
    /// Selected index (the 32-bit form mirrors it into two words).
    pub selected: u32,
    /// Selection marker word (the selection always writes [`SEL_MARK`]).
    pub sel_mark: u32,
    /// First hit rectangle's corners: `x0`, `y0`, `x1`, `y1`.
    pub rect0: (f32, f32, f32, f32),
    /// Second hit rectangle's corners: `x0`, `y0`, `x1`, `y1`.
    pub rect1: (f32, f32, f32, f32),
    /// Lower marker bound (unsigned on every use).
    pub bound_lo: u32,
    /// Upper marker bound (unsigned on every use).
    pub bound_hi: u32,
    /// Time-range top.
    pub hi: f32,
    /// Time-range bottom.
    pub lo: f32,
    /// Slot total: the index mappers' divisor, the scans' fallback, and
    /// the cursor store's generation word.
    pub total: u32,
}

impl ReplayBar {
    /// A zeroed bar: every float `+0.0`, no clock, no slots.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            seconds: 0.0,
            origin: 0.0,
            weight: 0.0,
            width: 0.0,
            cursor: 0,
            clock: None,
            slots: Vec::new(),
            selected: 0,
            sel_mark: 0,
            rect0: (0.0, 0.0, 0.0, 0.0),
            rect1: (0.0, 0.0, 0.0, 0.0),
            bound_lo: 0,
            bound_hi: 0,
            hi: 0.0,
            lo: 0.0,
            total: 0,
        }
    }

    /// Rounds a scaled position to a slot index without clamping.
    ///
    /// Answers `round_half_away((pos - lo) * total / (hi - lo))` with
    /// `total` converted as unsigned, every step in single precision in
    /// the 32-bit order (`total / span` first, times the offset). Results
    /// outside the `i32` range, and NaN, answer `i32::MIN`, matching the
    /// original's truncate conversion.
    #[must_use]
    pub fn scaled_index(&self, pos: f32) -> i32 {
        let span = self.hi - self.lo;
        let mut v = (self.total as f32) / span;
        let off = pos - self.lo;
        v *= off;
        round_half_away_truncate(v)
    }

    /// Rounds a scaled position to a slot index with clamping.
    ///
    /// Clamps `pos` into `[lo, hi]` first (a NaN input keeps its value
    /// through both comparisons), then, when `second_clamp` holds, into
    /// the cross-rectangle span `[rect0.x1, rect1.x0]`; scales and rounds
    /// like [`scaled_index`](Self::scaled_index) except the product runs
    /// `(offset) * (total / span)`. The 32-bit flag tests only its low
    /// byte (the proof pins that); the lift takes the decided bool.
    #[must_use]
    pub fn clamped_index(&self, pos: f32, second_clamp: bool) -> i32 {
        let span = self.hi - self.lo;
        let ratio = (self.total as f32) / span;
        let mut v = pos;
        if self.lo > v {
            v = self.lo;
        }
        if v > self.hi {
            v = self.hi;
        }
        if second_clamp {
            let lo2 = self.rect0.2;
            let hi2 = self.rect1.0;
            if lo2 > v {
                v = lo2;
            }
            if v > hi2 {
                v = hi2;
            }
        }
        v -= self.lo;
        v *= ratio;
        round_half_away_truncate(v)
    }

    /// Finds the last slot whose stamp does not exceed the upper bound.
    ///
    /// Scans from the end and answers the highest index with
    /// `stamp <= bound_hi`, or `None` when there are no slots or every
    /// stamp exceeds it (the 32-bit form answers -1 there).
    #[must_use]
    pub fn find_slot(&self) -> Option<usize> {
        for (i, slot) in self.slots.iter().enumerate().rev() {
            if slot.stamp <= self.bound_hi {
                return Some(i);
            }
        }
        None
    }

    /// Converts the seconds field to truncated milliseconds.
    ///
    /// Multiplies by 1000 in single precision and truncates toward zero.
    /// Unrepresentable results (NaN or past the `i64` range) answer 0,
    /// matching the original's truncate-to-qword conversion whose low
    /// dword is returned.
    #[must_use]
    pub fn millis_rounded(&self) -> u32 {
        let v = self.seconds * 1000.0;
        if v.is_nan() || v >= 9_223_372_036_854_775_808.0 || v <= -9_223_372_036_854_775_808.0 {
            0
        } else {
            (v as i64) as u32
        }
    }

    /// Tests whether a point lies strictly inside the bar rectangle.
    ///
    /// The rectangle spans `x` in `(rect0.x1, rect1.x0)` and `y` in
    /// `(rect0.y0, rect0.y1)`; every comparison is strict, so any NaN
    /// coordinate fails.
    #[must_use]
    pub fn hit_test(&self, x: f32, y: f32) -> bool {
        x > self.rect0.2 && self.rect1.0 > x && y > self.rect0.1 && self.rect0.3 > y
    }

    /// Tests whether a point lies inside the region selected by `mode`.
    ///
    /// Mode 0 uses the first rectangle, mode 1 the second; any other mode
    /// answers false without reading the point. Hits are strict inside on
    /// all four sides, so any NaN fails.
    #[must_use]
    pub fn region_hit_test(&self, x: f32, y: f32, mode: u32) -> bool {
        let (x0, y0, x1, y1) = if mode == 0 {
            self.rect0
        } else if mode == 1 {
            self.rect1
        } else {
            return false;
        };
        x > x0 && x1 > x && y > y0 && y1 > y
    }

    /// Stores the cursor position and generation, answering the generation.
    ///
    /// Writes `generation` to the total word and the position bits to the
    /// cursor word.
    pub fn store_cursor(&mut self, pos: f32, generation: u32) -> u32 {
        self.total = generation;
        self.cursor = pos.to_bits();
        generation
    }

    /// Scales a progress ratio by the bar width and the selected time base.
    ///
    /// Answers `(a / total) * width + origin`, all in single precision in
    /// the 32-bit order, times the selected denominator word converted as
    /// signed (`a` and `total` convert as unsigned). The selector picks
    /// the wide word when true, the narrow word otherwise. The 32-bit form
    /// widens the answer to `f64`, which is exact (the proof compares the
    /// widened bits).
    pub fn scaled_position(&self, a: u32, select: &mut impl BaseSelect, bases: &TimeBases) -> f32 {
        let q = (a as f32) / (self.total as f32);
        let den = if select.wide() {
            bases.den_wide
        } else {
            bases.den_narrow
        };
        let mut r = self.width * q;
        r += self.origin;
        r *= den.cast_signed() as f32;
        r
    }

    /// Scans forward for the first entry scoring above `thresh`.
    ///
    /// Scores each slot's stamp in order; on the first score strictly
    /// above `thresh` the entry is scored again and the second score is
    /// answered with the entry's tag. When nothing scores above it the
    /// total is answered with the default tag. Answers `(score, tag)`.
    pub fn scan_forward(&self, thresh: u32, score: &mut impl ScoreStamp) -> (u32, u32) {
        for slot in &self.slots {
            let v = score.score(slot.stamp);
            if v > thresh {
                let v2 = score.score(slot.stamp);
                return (v2, u32::from(slot.tag));
            }
        }
        (self.total, SCAN_DEFAULT_TAG)
    }

    /// Scans backward for the first entry scoring below `thresh`.
    ///
    /// Scores each slot's stamp from the end; on the first score strictly
    /// below `thresh` the entry is scored again and the second score is
    /// answered with the entry's tag and stamp. When nothing scores below
    /// it 0 is answered with the default tag and a zero stamp. Answers
    /// `(score, tag, stamp)`.
    pub fn scan_backward(&self, thresh: u32, score: &mut impl ScoreStamp) -> (u32, u32, u32) {
        for slot in self.slots.iter().rev() {
            let v = score.score(slot.stamp);
            if v < thresh {
                let v2 = score.score(slot.stamp);
                return (v2, u32::from(slot.tag), slot.stamp);
            }
        }
        (0, SCAN_DEFAULT_TAG, 0)
    }

    /// Publishes the slot ratio and the two marker scores.
    ///
    /// Answers `(total / (hi - lo), score(bound_lo), score(bound_hi))` in
    /// single precision, scoring the lower marker before the upper one.
    pub fn measure_slots(&self, score: &mut impl ScoreStamp) -> (f32, u32, u32) {
        let span = self.hi - self.lo;
        let ratio = (self.total as f32) / span;
        let first = score.score(self.bound_lo);
        let second = score.score(self.bound_hi);
        (ratio, first, second)
    }

    /// Publishes a sampled reading over the slot span plus the markers.
    ///
    /// Answers `(sample / (hi - lo), bound_lo, bound_hi)`. The 32-bit form
    /// also answers the third output pointer, which carries no meaning
    /// (the proof pins it).
    pub fn slot_ratio(&self, sample: &mut impl Sample) -> (f32, u32, u32) {
        let v = sample.sample();
        let span = self.hi - self.lo;
        let q = (v as f32) / span;
        (q, self.bound_lo, self.bound_hi)
    }

    /// Clamps a bound against one sample, selected by `which`.
    ///
    /// The sample is always taken first, even for an unknown `which`. When
    /// `which` is 0 the lower bound becomes `min(value, sample)` and 0 is
    /// answered; when 1 the upper bound becomes `max(value, sample)` and
    /// the new bound is answered; otherwise nothing is stored and `which`
    /// is answered back.
    pub fn clamp_bound(&mut self, value: u32, which: u32, sample: &mut impl Sample) -> u32 {
        let t = sample.sample();
        if which == 0 {
            let d = if value >= t { t } else { value };
            self.bound_lo = d;
            0
        } else if which == 1 {
            let e = if value <= t { t } else { value };
            self.bound_hi = e;
            e
        } else {
            which
        }
    }

    /// Selects slot `idx` and publishes its stamp when new.
    ///
    /// Runs the selection watcher when the selection changed, records the
    /// index with the marker word, then compares the slot's stamp against
    /// two samples: on each strict difference (above the first, below the
    /// second) the publish code goes out ([`PUBLISH_ABOVE`]/[`PUBLISH_BELOW`])
    /// and the stamp lands in the shared publish words with the flag bit
    /// set. Answers the published stamp when either publish path ran, else
    /// the second sample — or `None` when there are no slots (the 32-bit
    /// form answers the table address there, which the proof rebuilds and
    /// compares).
    ///
    /// # Panics
    ///
    /// When `idx` is past the slots (the original reads past the table).
    pub fn select_entry(
        &mut self,
        idx: u32,
        watch: &mut impl SelectionWatch,
        stamps: &mut impl Sample,
        publish: &mut impl Publish,
        state: &mut StampPublish,
    ) -> Option<u32> {
        if self.slots.is_empty() {
            return None;
        }
        if self.selected != idx {
            watch.selection_changed();
        }
        self.selected = idx;
        self.sel_mark = SEL_MARK;
        let slot = self.slots.get(idx as usize).unwrap_or_else(|| {
            panic!(
                "select slot {idx} past {} slots: the original reads past the table",
                self.slots.len()
            )
        });
        let stamp = slot.stamp;
        let v1 = stamps.sample();
        let above = stamp > v1;
        let v2 = stamps.sample();
        let below = stamp < v2;
        let mut ret = v2;
        if above {
            publish.publish(PUBLISH_ABOVE);
            state.flags |= 1;
            state.stamp = stamp;
            ret = stamp;
        }
        if below {
            publish.publish(PUBLISH_BELOW);
            state.flags |= 1;
            state.stamp = stamp;
            ret = stamp;
        }
        Some(ret)
    }

    /// Combines the clock reading with the selected time bases.
    ///
    /// Answers `(17 - ticks + num * weight) / num` in single precision in
    /// the 32-bit order, where `ticks` is the clock's slot `+0x24`
    /// reading, each numerator is picked by one selector sample (wide when
    /// true), and every word converts as signed. Answers `+0.0` without
    /// any call when the clock link is empty. The 32-bit form widens the
    /// answer to `f64`, which is exact (the proof compares the widened
    /// bits).
    pub fn time_factor(
        &self,
        clock: &mut impl ClockRead,
        select: &mut impl BaseSelect,
        bases: &TimeBases,
    ) -> f32 {
        if self.clock.is_none() {
            return 0.0;
        }
        let num1 = if select.wide() {
            bases.num_wide
        } else {
            bases.num_narrow
        };
        let ticks = clock.clock_b();
        let mut x = TICKS_BASE - (ticks.cast_signed() as f32);
        let adj = (num1.cast_signed() as f32) * self.weight;
        x += adj;
        let num2 = if select.wide() {
            bases.num_wide
        } else {
            bases.num_narrow
        };
        x /= num2.cast_signed() as f32;
        x
    }
}

/// Rounds half away from zero, then truncates to `i32` like the original.
///
/// Out-of-range results and NaN answer `i32::MIN`, matching the 32-bit
/// truncate conversion; the sign test keeps its NaN path (NaN takes the
/// upper branch and lands on `MIN` through the guard).
fn round_half_away_truncate(v: f32) -> i32 {
    let r = if 0.0 > v { v - 0.5 } else { v + 0.5 };
    if r.is_nan() || r >= 2_147_483_648.0 || r < -2_147_483_648.0 {
        i32::MIN
    } else {
        r as i32
    }
}
