//! The altitude gate: one conversion head with five registration tails.
//!
//! Lifted from the eight verified `script_vm_altitude_pack_*` rewrites.
//! Every routine compares its altitude word against the threshold with
//! ordered-compare semantics (an unordered comparison keeps the altitude,
//! so NaN is never converted), converts a low altitude through the ground
//! query, packs the triple and hands it to its tail. The two restart
//! routines share one body, the three single-register routines share
//! another; each instance is proved against its tail's method.

// Signatures mirror the 32-bit routines' words one by one, so long
// argument lists are inherent here.
#![allow(clippy::too_many_arguments)]

use core::cmp::Ordering;

/// Mode word the head passes to the ground query with (`x`, `y`).
pub const CONV_MODE: u32 = 4;

/// Converts (`x`, `y`) to a ground altitude: the head's query callee.
///
/// Words are single-precision bits in both directions; the query's
/// answer passes through to the packed triple unchanged.
pub trait GroundQuery {
    /// Answers the ground altitude bits for (`x`, `y`) in `mode`.
    fn ground_z(&mut self, x: u32, y: u32, mode: u32) -> u32;
}

impl<F: FnMut(u32, u32, u32) -> u32> GroundQuery for F {
    fn ground_z(&mut self, x: u32, y: u32, mode: u32) -> u32 {
        self(x, y, mode)
    }
}

/// Registers a point with two trailing words: the restart tails.
pub trait SinkReg {
    /// Registers `point` with (`w`, `extra`).
    fn register(&mut self, point: [u32; 3], w: u32, extra: u32);
}

/// Clears an area with five trailing words: the full-area tail.
pub trait SinkClear6 {
    /// Clears `point` with (`w`, `extra`, `z0`, `z1`, `z2`).
    fn clear(&mut self, point: [u32; 3], w: u32, extra: u32, z0: u32, z1: u32, z2: u32);
}

/// Emits a point with four trailing words: the flag tails.
pub trait SinkTail4 {
    /// Emits `point` with (`w`, `a`, `b`, `c`).
    fn emit(&mut self, point: [u32; 3], w: u32, a: u32, b: u32, c: u32);
}

/// Registers a point with one trailing word: the single-register tails.
pub trait SinkReg2 {
    /// Registers `point` with `w`.
    fn register(&mut self, point: [u32; 3], w: u32);
}

impl<F: FnMut([u32; 3], u32, u32)> SinkReg for F {
    fn register(&mut self, point: [u32; 3], w: u32, extra: u32) {
        self(point, w, extra);
    }
}

impl<F: FnMut([u32; 3], u32, u32, u32, u32, u32)> SinkClear6 for F {
    fn clear(&mut self, point: [u32; 3], w: u32, extra: u32, z0: u32, z1: u32, z2: u32) {
        self(point, w, extra, z0, z1, z2);
    }
}

impl<F: FnMut([u32; 3], u32, u32, u32, u32)> SinkTail4 for F {
    fn emit(&mut self, point: [u32; 3], w: u32, a: u32, b: u32, c: u32) {
        self(point, w, a, b, c);
    }
}

impl<F: FnMut([u32; 3], u32)> SinkReg2 for F {
    fn register(&mut self, point: [u32; 3], w: u32) {
        self(point, w);
    }
}

/// The altitude threshold the eight routines share.
///
/// The 32-bit routines read it from a global word; the lift owns the
/// bits, and each tail shape is a method that resolves the altitude
/// through the gate and then runs its tail.
#[derive(Debug, Clone, Copy)]
pub struct AltitudeGate {
    /// Threshold bits: altitudes at or below convert, above keep `z`.
    threshold: u32,
}

impl AltitudeGate {
    /// Builds the gate over the threshold word's bits.
    #[must_use]
    pub fn new(threshold: u32) -> Self {
        Self { threshold }
    }

    /// The threshold word's bits.
    #[must_use]
    pub fn threshold(&self) -> u32 {
        self.threshold
    }

    /// Resolves (`x`, `y`, `z`) to the packed triple.
    ///
    /// The comparison is ordered: when the threshold is below `z`, or
    /// either side is NaN, the altitude keeps `z`; otherwise the ground
    /// query's answer takes its place.
    fn resolve(self, query: &mut impl GroundQuery, x: u32, y: u32, z: u32) -> [u32; 3] {
        let limit = f32::from_bits(self.threshold);
        let alt = f32::from_bits(z);
        // The rewrite negates the ordered comparison; the match says the
        // same with the unordered arm explicit (NaN keeps `z`).
        let conv = if matches!(limit.partial_cmp(&alt), Some(Ordering::Less) | None) {
            z
        } else {
            Self::quiet_snan(query.ground_z(x, y, CONV_MODE))
        };
        [x, y, conv]
    }

    /// Quiets a signalling-NaN answer, as the x87 return path does.
    ///
    /// The 32-bit query callee returns its float through the x87 unit,
    /// which sets the quiet bit of a signalling NaN; the lift has no
    /// x87, so it sets the bit explicitly to stay bit for bit.
    fn quiet_snan(bits: u32) -> u32 {
        const EXP: u32 = 0x7F80_0000;
        const MANT: u32 = 0x007F_FFFF;
        const QUIET: u32 = 0x0040_0000;
        if bits & EXP == EXP && bits & MANT != 0 && bits & QUIET == 0 {
            bits | QUIET
        } else {
            bits
        }
    }

    /// Registers a restart point: the two restart tails.
    pub fn register_restart(
        &self,
        query: &mut impl GroundQuery,
        sink: &mut impl SinkReg,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
        extra: u32,
    ) {
        let point = self.resolve(query, x, y, z);
        sink.register(point, w, extra);
    }

    /// Clears an area: the full-area tail with three trailing zeros.
    pub fn clear_area(
        &self,
        query: &mut impl GroundQuery,
        sink: &mut impl SinkClear6,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
        extra: u32,
    ) {
        let point = self.resolve(query, x, y, z);
        sink.clear(point, w, extra, 0, 0, 0);
    }

    /// Clears cars from an area: the flag tail (`w`, 0, 1, 0).
    pub fn clear_area_cars(
        &self,
        query: &mut impl GroundQuery,
        sink: &mut impl SinkTail4,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        let point = self.resolve(query, x, y, z);
        sink.emit(point, w, 0, 1, 0);
    }

    /// Registers a point: the three single-register tails.
    pub fn register_point(
        &self,
        query: &mut impl GroundQuery,
        sink: &mut impl SinkReg2,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        let point = self.resolve(query, x, y, z);
        sink.register(point, w);
    }

    /// Clears objects from an area: the double tail, first (`w`), then
    /// (`w`, 0, 0, 0) through the second sink.
    pub fn clear_objects(
        &self,
        query: &mut impl GroundQuery,
        first: &mut impl SinkReg2,
        second: &mut impl SinkTail4,
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    ) {
        let point = self.resolve(query, x, y, z);
        first.register(point, w);
        second.emit(point, w, 0, 0, 0);
    }
}
