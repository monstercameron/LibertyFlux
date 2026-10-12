//! The blend method of the layout frame.
//!
//! The method stores a mode word in the frame. Any mode other than one then
//! clamps an amount into the range from zero to a shared upper bound (a
//! not-a-number passes through unchanged), blends it with two factors the
//! frame holds, in single precision, and forwards the mode and the blended
//! value's bits to the frame's blend handler. The handler is the frame's own
//! step, so it takes the host and the frame.
//!
//! The blend is written as the same four single-precision operations in the
//! same order as the original: `a * half`, `a * clamped`, `b - scaled` and
//! `difference + weighted`. Each operand goes through
//! [`core::hint::black_box`], so the compiler cannot swap the operands of an
//! operation: when both operands of an operation are not-a-number, the result
//! carries the first operand's payload, and the order is part of the answer.

use core::hint::black_box;

use super::frame::{UILayoutFrame, field};

/// The calls the blend method makes outside the frame's bytes.
pub trait BlendHost {
    /// The shared upper bound for the amount.
    fn clamp_high(&mut self) -> f32;

    /// The shared factor the frame's first factor is scaled by.
    fn half_factor(&mut self) -> f32;

    /// The frame's blend handler, with the mode and the blended value's bits.
    /// The method does not use its answer.
    fn blend_handler(&mut self, frame: &mut UILayoutFrame, mode: u32, blended_bits: u32) -> u32;
}

/// The mode the blend method leaves alone: it stores the mode and returns.
const MODE_THAT_SKIPS_BLEND: u32 = 1;

impl UILayoutFrame {
    /// Stores `mode` in the word at 0x14. Unless the mode is one, clamps
    /// `amount` (given as its bits) into zero to the shared upper bound, blends
    /// it with the factors at 0x8 and 0x10, and hands the mode and the blend to
    /// the blend handler. Answers zero: the forwarding path's answer is not
    /// part of this method's result.
    pub fn store_mode_then_blend_and_forward<H: BlendHost>(
        &mut self,
        host: &mut H,
        mode: u32,
        amount_bits: u32,
    ) -> u32 {
        self.set_word_at(field::SLOT_14, mode);
        if mode != MODE_THAT_SKIPS_BLEND {
            let high = host.clamp_high();
            let half = host.half_factor();

            // Ordered comparisons: a not-a-number fails both tests and keeps its bits.
            let mut clamped = f32::from_bits(amount_bits);
            if clamped < 0.0 {
                clamped = 0.0;
            } else if clamped > high {
                clamped = high;
            }

            let first = f32::from_bits(self.word_at(field::FLOAT_8));
            let second = f32::from_bits(self.word_at(field::BLOCK_10));
            let scaled = black_box(black_box(first) * black_box(half));
            let weighted = black_box(black_box(first) * black_box(clamped));
            let difference = black_box(black_box(second) - black_box(scaled));
            let blended = black_box(black_box(difference) + black_box(weighted));

            host.blend_handler(self, mode, blended.to_bits());
        }
        0
    }
}
