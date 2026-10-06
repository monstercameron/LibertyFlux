//! The software mixer voice: a voice rendered by the engine mixer.
//!
//! Lifted from the verified rewrites of `rage::audVoiceSoft`. The 32-bit
//! object is a header (flag byte, rate word, source and auxiliary engine
//! pointers, child voice, mixing buffer) followed by a ring of sample
//! lanes; here those are plain fields, with collaborator objects carried
//! as opaque cookies and reached through [`SoftWorld`].

use lf_core::Handle32;

use super::shared;
use super::{AuxTag, BufferTag, ChildTag};

/// One sample lane of the software voice's ring: the 14 copied parameter
/// words, the derived doubled count, and the source remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SoftLane {
    /// The 14 parameter words copied from the sample source.
    pub params: [u32; 14],
    /// Twice the scaled sample count.
    pub pos: u32,
    /// The source bias word minus [`SoftLane::pos`].
    pub rem: u32,
}

/// What the software voice needs from the engine around it: its child
/// voice, its own lifecycle entries, the rate/position converters, the
/// mixing buffer, and its state gate.
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies, constant zero
/// words dropped, and unused answers unmodelled (see the registry for the
/// per-method narrowings).
pub trait SoftWorld {
    /// The helper query over the child voice (the stopping check's middle
    /// arm); only the low byte of the answer is tested.
    fn child_query(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Stops the child voice; answers the child's answer.
    fn child_stop(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Resumes the child voice with the folded mode bit.
    fn child_resume(&mut self, child: Option<Handle32<ChildTag>>, mode: u32);
    /// Starts the child voice with the doubled converted rate.
    fn child_start(&mut self, child: Option<Handle32<ChildTag>>, rate: u32);
    /// Polls the child voice for its cursor or mode word.
    fn child_poll(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Shuts the child voice down.
    fn child_shutdown(&mut self, child: Option<Handle32<ChildTag>>);
    /// The voice's own start entry with an explicit mode.
    fn own_start(&mut self, mode: u32);
    /// The voice's own restart entry; answers its answer.
    fn restart(&mut self) -> u32;
    /// Converts a rate word against the voice rate.
    fn convert_rate(&mut self, word: u32, base: u32) -> u32;
    /// Relays the level word.
    fn report_level(&mut self, level: u32);
    /// Releases the mixing buffer.
    fn free_buffer(&mut self, buffer: Option<Handle32<BufferTag>>);
    /// The base teardown entry; answers its answer.
    fn base_teardown(&mut self) -> u32;
    /// The voice's own state gate (its table slot); only the low byte of
    /// the answer is tested.
    fn state_gate(&mut self) -> u32;
    /// The voice's own mode-refresh fallback slot.
    fn mode_fallback(&mut self);
    /// Converts a combined cursor against the voice rate; answers the
    /// playback position.
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32;
    /// Refines the incoming sample count against the voice rate.
    fn refine_count(&mut self, count: u32, rate: u32) -> u32;
    /// Consumes samples through the auxiliary engine object.
    fn consume_samples(&mut self, aux: Option<Handle32<AuxTag>>, count: u32) -> u32;
}

/// A software mixer voice, owning its header words and its sample lanes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoftVoice {
    /// The flag byte.
    pub flags: u8,
    /// The sample-rate word.
    pub rate: u32,
    /// The auxiliary engine object the queueing method consumes through.
    pub aux: Option<Handle32<AuxTag>>,
    /// The level word read through the parameter block.
    pub level: u32,
    /// The status byte read through the parameter block.
    pub params_status: u8,
    /// The child voice.
    pub child: Option<Handle32<ChildTag>>,
    /// The mixing buffer handle.
    pub buffer: Option<Handle32<BufferTag>>,
    /// The ring of sample lanes.
    pub lanes: Vec<SoftLane>,
    /// The ring cursor: index of the current lane.
    pub cursor: u32,
    /// The ring divisor; the cursor advances modulo this.
    pub divisor: u32,
    /// The tuning word: the refined count the queueing method stores, also
    /// the bias the position method adds.
    pub tuning: u32,
    /// The frequency word the position method scales.
    pub freq: u32,
    /// The expected child mode bit the refresh compares against.
    pub mode_cmp: u32,
    /// The mode-refresh counter the refresh decrements.
    pub mode_count: u32,
    /// The mode-refresh selector byte.
    pub mode_byte: u8,
}

impl SoftVoice {
    /// True when the voice shows the stopping states: both the stopped
    /// and stopping bits, or the resume-pending bit, or a nonzero low
    /// byte from the child query, or the stopping bit of the parameter
    /// status byte.
    pub fn is_stopping<W: SoftWorld>(&self, world: &mut W) -> bool {
        if self.flags & shared::FLAG_STOPPED != 0
            && self.flags & shared::FLAG_STOPPING != 0
        {
            return true;
        }
        if self.flags & shared::FLAG_RESUME != 0 {
            return true;
        }
        if world.child_query(self.child) & 0xFF != 0 {
            return true;
        }
        self.params_status & 0x40 != 0
    }

    /// True only when the synth path is active and the current lane's
    /// remainder is zero.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last lane (the original reads
    /// whatever word lies there).
    pub fn lane_rest_quiet(&self) -> bool {
        if self.flags & shared::FLAG_SYNTH == 0 {
            return false;
        }
        self.lanes[self.cursor as usize].rem == 0
    }

    /// Stops the voice: clears the stopped and resume-pending bits, then
    /// stops the child and answers its answer.
    pub fn stop<W: SoftWorld>(&mut self, world: &mut W) -> u32 {
        self.flags &= !(shared::FLAG_STOPPED | shared::FLAG_RESUME);
        world.child_stop(self.child)
    }

    /// Resumes the voice on its child, unless resume is not pending.
    /// Otherwise runs the restart entry first, resumes the child with the
    /// mode bit folded out of the flags, and clears the pending bit.
    pub fn resume<W: SoftWorld>(&mut self, world: &mut W) {
        if self.flags & shared::FLAG_RESUME == 0 {
            return;
        }
        world.restart();
        let mode = shared::resume_mode(self.flags);
        world.child_resume(self.child, mode);
        self.flags &= !shared::FLAG_RESUME;
    }

    /// Starts the voice on its child and reports the level state in the
    /// flags: on the synth path the voice's own start entry runs with
    /// mode 1, otherwise the rate word converts against the voice rate
    /// and the child starts with twice that answer. Either way the level
    /// relays, then a zero level sets the stopped and stopping bits while
    /// any other level (NaN counts as nonzero) sets the resume-pending
    /// bit.
    pub fn start<W: SoftWorld>(&mut self, world: &mut W, rate_word: u32) {
        if self.flags & shared::FLAG_SYNTH != 0 {
            world.own_start(1);
        } else {
            let ans = world.convert_rate(rate_word, self.rate);
            world.child_start(self.child, ans.wrapping_mul(2));
        }
        world.report_level(self.level);
        if f32::from_bits(self.level) != 0.0 {
            self.flags |= shared::FLAG_RESUME;
        } else {
            self.flags |= shared::FLAG_STOPPED | shared::FLAG_STOPPING;
        }
    }

    /// Tears the voice down: shuts a present child down and clears it,
    /// releases the mixing buffer on the synth path and clears it, then
    /// runs the base teardown and answers its answer.
    pub fn teardown<W: SoftWorld>(&mut self, world: &mut W) -> u32 {
        if self.child.is_some() {
            world.child_shutdown(self.child);
            self.child = None;
        }
        if self.flags & shared::FLAG_SYNTH != 0 {
            world.free_buffer(self.buffer);
            self.buffer = None;
        }
        world.base_teardown()
    }

    /// Refreshes the child mode, relays the level, then runs the restart
    /// entry and answers its answer. On the synth path the child polls
    /// and its answer reduces to one bit at the 0x10000 boundary; on a
    /// mismatch with the expected bit the mode refreshes directly
    /// (decrementing the counter when the selector byte is set) or
    /// through the fallback slot.
    pub fn refresh<W: SoftWorld>(&mut self, world: &mut W) -> u32 {
        if self.flags & shared::FLAG_SYNTH != 0 {
            let ans = world.child_poll(self.child);
            let bit = u32::from(ans >= 0x10000);
            if bit != self.mode_cmp {
                if self.mode_byte == 0 || self.mode_count > 0 {
                    world.own_start(0);
                    if self.mode_byte != 0 {
                        self.mode_count = self.mode_count.wrapping_sub(1);
                    }
                } else {
                    world.mode_fallback();
                }
            }
        }
        world.report_level(self.level);
        world.restart()
    }

    /// The current playback position, or all-ones when the state gate
    /// reports the voice is not playing. Otherwise the child polls and
    /// its answer combines with the scaled frequency word and the tuning
    /// bias, converted against the voice rate.
    pub fn position<W: SoftWorld>(&mut self, world: &mut W) -> u32 {
        if world.state_gate() as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let ans = world.child_poll(self.child);
        let scaled = shared::position_arg(self.freq, ans, self.tuning);
        world.convert_position(scaled, self.rate)
    }

    /// Refreshes the current ring lane and advances the cursor, answering
    /// the division quotient of the step. Copies the 14 source words
    /// into the lane, optionally refines the sample count through the
    /// engine, derives the doubled scaled count and the remainder, and
    /// steps the cursor modulo the divisor.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last lane, or the divisor is
    /// zero (the original divides there too).
    pub fn refresh_lane<W: SoftWorld>(
        &mut self,
        world: &mut W,
        src: &[u32; 14],
        src_bias: u32,
        count: u32,
    ) -> u32 {
        let idx = self.cursor as usize;
        self.lanes[idx].params.copy_from_slice(src);
        let mut n = count;
        if n != 0 && self.flags & shared::FLAG_SYNTH != 0 {
            let refined = world.refine_count(n, self.rate);
            self.tuning = refined;
            let took = world.consume_samples(self.aux, n);
            if (took as i32) > 0 {
                n = n.wrapping_sub(took);
            }
        }
        let mut x5 = shared::u32_to_f32_bias(self.rate) * shared::u32_to_f32_bias(n);
        x5 *= shared::MILLI;
        let x1 = shared::round_down_magic_soft(x5);
        let t = shared::trunc_f32_to_i64_lo(x1);
        let pos = t.wrapping_add(t);
        self.lanes[idx].pos = pos;
        self.lanes[idx].rem = src_bias.wrapping_sub(pos);
        let next = self.cursor.wrapping_add(1);
        self.cursor = next % self.divisor;
        next / self.divisor
    }
}
