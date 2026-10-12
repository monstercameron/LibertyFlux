//! The PC ADPCM voice: ADPCM synthesis through the engine mixer.
//!
//! Lifted from the verified rewrites of `rage::audVoicePcAdpcm`. Its
//! lifecycle entries are the software voice's at shifted offsets (the
//! child sits one slot further into the object); its queueing entry runs
//! the ADPCM synth path with codec and predictor tables, and its position
//! entry measures against the mixer on the synth path. Collaborators are
//! reached through [`PcWorld`].

use lf_core::Handle32;

use super::shared;
use super::{BufferTag, ChildTag, CodecTag, MixerTag};

/// One ADPCM sample lane: the 14 copied parameter words, the synth
/// accumulator, and the source remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PcLane {
    /// The 14 parameter words copied from the sample source.
    pub params: [u32; 14],
    /// The synth accumulator (zero when the synth branch does not run).
    pub acc: u32,
    /// The source bias word minus [`PcLane::acc`].
    pub rem: u32,
}

/// What the PC ADPCM voice needs from the engine around it: the same
/// child, lifecycle, buffer and gate roles as the software voice, plus
/// the ADPCM codec, the mixer, and the child measurement.
pub trait PcWorld {
    /// The helper query over the child voice; only the low byte of the
    /// answer is tested.
    fn child_query(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Stops the child voice; answers the child's answer.
    fn child_stop(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Resumes the child voice with the folded mode bit.
    fn child_resume(&mut self, child: Option<Handle32<ChildTag>>, mode: u32);
    /// Starts the child voice with the doubled converted rate.
    fn child_start(&mut self, child: Option<Handle32<ChildTag>>, rate: u32);
    /// Polls the child voice for its cursor or mode word.
    fn child_poll(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
    /// Measures the child voice for the mixer-relative position.
    fn child_measure(&mut self, child: Option<Handle32<ChildTag>>) -> u32;
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
    /// The voice's own state gate; only the low byte of the answer is
    /// tested.
    fn state_gate(&mut self) -> u32;
    /// The voice's own mode-refresh fallback slot.
    fn mode_fallback(&mut self);
    /// Converts a combined cursor against a base; answers the position.
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32;
    /// Resolves a synth argument through the codec to a table entry.
    fn resolve_codec(&mut self, codec: Option<Handle32<CodecTag>>, arg: u32) -> u32;
    /// Asks the mixer singleton for its cursor.
    fn mixer_cursor(&mut self, mixer: Option<Handle32<MixerTag>>) -> u32;
}

/// A PC ADPCM voice, owning its header words, its sample lanes, and the
/// ADPCM codec and predictor tables its queueing entry reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PcAdpcmVoice {
    /// The flag byte.
    pub flags: u8,
    /// The sample-rate word.
    pub rate: u32,
    /// The ADPCM codec the queueing method resolves through.
    pub codec: Option<Handle32<CodecTag>>,
    /// The level word read through the parameter block.
    pub level: u32,
    /// The status byte read through the parameter block.
    pub params_status: u8,
    /// The child voice.
    pub child: Option<Handle32<ChildTag>>,
    /// The mixing buffer handle.
    pub buffer: Option<Handle32<BufferTag>>,
    /// The ring of ADPCM sample lanes.
    pub lanes: Vec<PcLane>,
    /// The ring cursor: index of the current lane.
    pub cursor: u32,
    /// The ring divisor; the cursor advances modulo this.
    pub divisor: u32,
    /// The tuning word: the converted rate the queueing method stores,
    /// also the bias and lead rate the position method converts.
    pub tuning: u32,
    /// The frequency word the position method scales.
    pub freq: u32,
    /// The expected child mode bit the refresh compares against.
    pub mode_cmp: u32,
    /// The mode-refresh counter the refresh decrements.
    pub mode_count: u32,
    /// The mode-refresh selector byte.
    pub mode_byte: u8,
    /// The mixer's cursor-scaling word, snapshotted by the caller.
    pub mixer_word: u32,
    /// The mixer singleton the position method asks for its cursor.
    pub mixer: Option<Handle32<MixerTag>>,
    /// The codec table the queueing method indexes by the resolved
    /// entry (two words per entry; the first is read).
    pub codec_table: Vec<u32>,
    /// The predictor table the queueing method fetches triples from
    /// (two bytes plus one).
    pub predictor: Vec<u8>,
    /// The fetched predictor pair.
    pub out_word: u16,
    /// The fetched predictor trailing byte.
    pub out_byte: u8,
}

impl PcAdpcmVoice {
    /// True when the voice shows the stopping states: both the stopped
    /// and stopping bits, or the resume-pending bit, or a nonzero low
    /// byte from the child query, or the stopping bit of the parameter
    /// status byte.
    pub fn is_stopping<W: PcWorld>(&self, world: &mut W) -> bool {
        if self.flags & shared::FLAG_STOPPED != 0 && self.flags & shared::FLAG_STOPPING != 0 {
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

    /// Stops the voice: clears the stopped and resume-pending bits, then
    /// stops the child and answers its answer.
    pub fn stop<W: PcWorld>(&mut self, world: &mut W) -> u32 {
        self.flags &= !(shared::FLAG_STOPPED | shared::FLAG_RESUME);
        world.child_stop(self.child)
    }

    /// Resumes the voice on its child, unless resume is not pending.
    /// Otherwise runs the restart entry first, resumes the child with the
    /// mode bit folded out of the flags, and clears the pending bit.
    pub fn resume<W: PcWorld>(&mut self, world: &mut W) {
        if self.flags & shared::FLAG_RESUME == 0 {
            return;
        }
        world.restart();
        let mode = shared::resume_mode(self.flags);
        world.child_resume(self.child, mode);
        self.flags &= !shared::FLAG_RESUME;
    }

    /// Starts the voice on its child and reports the level state in the
    /// flags, like the software voice: the synth path runs the voice's
    /// own start entry with mode 1, otherwise the rate word converts and
    /// the child starts with twice that answer; then a zero level sets
    /// the stopped and stopping bits while any other level (NaN counts
    /// as nonzero) sets the resume-pending bit.
    pub fn start<W: PcWorld>(&mut self, world: &mut W, rate_word: u32) {
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
    /// always releases the mixing buffer and clears it, then runs the
    /// base teardown and answers its answer. Unlike the software voice,
    /// the buffer release is not gated on the synth bit.
    pub fn teardown<W: PcWorld>(&mut self, world: &mut W) -> u32 {
        if self.child.is_some() {
            world.child_shutdown(self.child);
            self.child = None;
        }
        world.free_buffer(self.buffer);
        self.buffer = None;
        world.base_teardown()
    }

    /// Refreshes the child mode, relays the level, then runs the restart
    /// entry and answers its answer, like the software voice.
    pub fn refresh<W: PcWorld>(&mut self, world: &mut W) -> u32 {
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
    /// reports the voice is not playing. On the synth path the child is
    /// measured and the mixer asked for its cursor, and the answer is
    /// the converted tuning rate plus the converted non-negative part
    /// of (measure minus cursor); otherwise the child polls and its
    /// answer combines with the scaled frequency word and the tuning
    /// bias, converted against the voice rate.
    pub fn position<W: PcWorld>(&mut self, world: &mut W) -> u32 {
        if world.state_gate() as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        if self.flags & shared::FLAG_SYNTH != 0 {
            let m = world.child_measure(self.child);
            let c = world.mixer_cursor(self.mixer);
            let lead = if m >= c { m.wrapping_sub(c) } else { 0 };
            let p = world.convert_position(self.tuning, self.rate);
            let q = world.convert_position(lead, self.mixer_word);
            p.wrapping_add(q)
        } else {
            let ans = world.child_poll(self.child);
            let scaled = shared::position_arg(self.freq, ans, self.tuning);
            world.convert_position(scaled, self.rate)
        }
    }

    /// Queues one ADPCM synthesis block into the current lane and
    /// advances the cursor. Copies the 14 source words, clears the
    /// accumulator, and for a nonzero argument on the synth path
    /// resolves the codec entry, converts the rate, fetches the
    /// predictor triple, and sets the accumulator from the codec table
    /// word; the tail always stores the remainder and steps the cursor
    /// modulo the divisor.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last lane, the divisor is zero,
    /// or the resolved table entries fall outside the owned tables (the
    /// original reads whatever lies at those addresses).
    pub fn queue_block<W: PcWorld>(
        &mut self,
        world: &mut W,
        src: &[u32; 14],
        src_bias: u32,
        arg: u32,
    ) {
        let idx = self.cursor as usize;
        self.lanes[idx].params.copy_from_slice(src);
        self.lanes[idx].acc = 0;
        if arg != 0 && self.flags & shared::FLAG_SYNTH != 0 {
            let ans1 = world.resolve_codec(self.codec, arg);
            // The original reads one word eight bytes per entry into the
            // codec table; the byte offset wraps at 32 bits.
            let wi = ans1.wrapping_mul(8) / 4;
            let w = self.codec_table[wi as usize];
            let ans2 = world.convert_rate(arg, self.rate);
            self.tuning = ans2;
            let (acc, edx1) = shared::synth_acc(w, ans2);
            let pi = edx1.wrapping_mul(3) as usize;
            let lo = self.predictor[pi];
            let hi = self.predictor[pi + 1];
            self.out_word = u16::from_le_bytes([lo, hi]);
            self.out_byte = self.predictor[pi + 2];
            self.lanes[idx].acc = acc;
        }
        let acc = self.lanes[idx].acc;
        self.lanes[idx].rem = src_bias.wrapping_sub(acc);
        self.cursor = self.cursor.wrapping_add(1) % self.divisor;
    }
}
