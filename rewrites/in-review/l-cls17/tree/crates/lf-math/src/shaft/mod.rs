//! Shaft dynamics: four channel steps of the engine's spatial shaft class.
//!
//! The original object carries one calibration gain, two calibration
//! pairs (a scale and a divisor each), a table of twelve floats (four rows
//! of three columns) and a flag byte. Each of its four step methods takes
//! the step length in seconds and advances one channel: it stores the
//! length, works out three direction seeds from the calibration and the
//! step, mixes the seeds with the table into a set of mixed values and a
//! total, and clears the object's pending flag. The step returns the third
//! seed.
//!
//! Each channel owns its own words (its stored length, seeds, residual,
//! mixed values and total); the four channels share the gain, the table,
//! the flag and the pending flag. Channels 0 and 1 use the first
//! calibration pair and channels 2 and 3 the second. Channel 2 and channel 3
//! negate the offset that the other two do not, and their seeds fall on
//! different axes; the per-channel differences are in [`steps`].
//!
//! The square root that the original takes through a helper is a callee
//! here: [`ShaftHost::root`] answers it, and every step calls it at the
//! same point and with the same radicand as the original. The original
//! helper's own effects are not known, so the host is free to answer it
//! however the port does (the differential test uses the host square
//! root on both sides).
//!
//! Every product and sum keeps the operand order of the verified
//! behaviour: a not-a-number takes its payload and sign from the first
//! operand, so the order is pinned with [`core::hint::black_box`] (see the
//! `steps` module). Floats are bit for bit.
//!
//! What is lifted, and what each proof narrows away, is recorded in
//! [`registry`]. The class constructor and the matrix update are not
//! lifted: their rewrites are not verified yet.

mod registry;
mod steps;

#[cfg(test)]
mod tests;

pub use registry::{PROOFS, Proof};

/// The square-root helper the original calls, and any other callee the
/// step methods need. One method per callee; no other effect is modelled.
pub trait ShaftHost {
    /// Square root of a non-zero radicand (the original's helper takes the
    /// radicand and answers one float).
    fn root(&mut self, radicand: f32) -> f32;
}

/// One calibration pair: the scale and the divisor a step uses.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Calibration {
    /// Multiplies twice the gain; the step's offset is proportional to it.
    pub scale: f32,
    /// The step length is divided by this before it scales the offset.
    pub divisor: f32,
}

/// The state one channel keeps between steps.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Channel {
    /// The step length the channel last advanced by.
    pub input: f32,
    /// The three direction seeds of the last step.
    pub seeds: [f32; 3],
    /// The scalar that combines the seeds with the offset.
    pub residual: f32,
    /// The three seeds mixed with the table, one per table column.
    pub mixed: [f32; 3],
    /// The final combination of the mixed values.
    pub total: f32,
}

/// A shaft object: calibration, table and the four channels.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shaft {
    /// The shared gain. Its sign is part of the seeds (see [`steps`]).
    pub gain: f32,
    /// The calibration pairs: `[0]` for channels 0 and 1, `[1]` for
    /// channels 2 and 3.
    pub calibration: [Calibration; 2],
    /// Four rows of three columns. The first three rows mix the seeds; the
    /// fourth row is the additive term of the mix.
    pub table: [[f32; 3]; 4],
    /// When set, the seeds are the channel's fixed axis and the length is
    /// not taken: the direction is not normalised.
    pub seeds_fixed: bool,
    /// Set by a step's original; a step clears it. Nothing here reads it.
    pub pending: bool,
    /// The four channels.
    pub channels: [Channel; 4],
}

impl Shaft {
    /// Advances channel 0 by `dt` and returns its third seed.
    ///
    /// Uses the first calibration pair. The seeds are the gain axis (with
    /// the gain negated), the cross term, and the offset axis (negated).
    /// With the seeds fixed they are the unit vector on the first axis.
    pub fn advance_0(&mut self, host: &mut impl ShaftHost, dt: f32) -> f32 {
        let head = steps::head_0(self.gain, self.calibration[0], self.seeds_fixed, host, dt);
        self.finish(0, dt, head)
    }

    /// Advances channel 1 by `dt` and returns its third seed.
    ///
    /// Uses the first calibration pair. The seeds are the gain axis (not
    /// negated), the cross term, and the offset axis (not negated). With
    /// the seeds fixed they are minus the unit vector on the first axis.
    pub fn advance_1(&mut self, host: &mut impl ShaftHost, dt: f32) -> f32 {
        let head = steps::head_1(self.gain, self.calibration[0], self.seeds_fixed, host, dt);
        self.finish(1, dt, head)
    }

    /// Advances channel 2 by `dt` and returns its third seed.
    ///
    /// Uses the second calibration pair and the offset with its sign
    /// reversed. The seeds are the cross term, the gain axis (the gain
    /// times minus one) and the offset axis. With the seeds fixed they are
    /// minus the unit vector on the second axis.
    pub fn advance_2(&mut self, host: &mut impl ShaftHost, dt: f32) -> f32 {
        let head = steps::head_2(self.gain, self.calibration[1], self.seeds_fixed, host, dt);
        self.finish(2, dt, head)
    }

    /// Advances channel 3 by `dt` and returns its third seed.
    ///
    /// Uses the second calibration pair and the offset with its sign
    /// reversed. The seeds are the cross term, the gain axis (negated) and
    /// the offset axis. With the seeds fixed they are the unit vector on
    /// the second axis.
    pub fn advance_3(&mut self, host: &mut impl ShaftHost, dt: f32) -> f32 {
        let head = steps::head_3(self.gain, self.calibration[1], self.seeds_fixed, host, dt);
        self.finish(3, dt, head)
    }

    /// Stores one step's results in the channel and clears the pending flag.
    fn finish(&mut self, index: usize, dt: f32, head: steps::Head) -> f32 {
        let (mixed, total) = steps::mix(&self.table, head.seeds, head.residual);
        let channel = &mut self.channels[index];
        channel.input = dt;
        channel.seeds = head.seeds;
        channel.residual = head.residual;
        channel.mixed = mixed;
        channel.total = total;
        self.pending = false;
        head.seeds[2]
    }
}
