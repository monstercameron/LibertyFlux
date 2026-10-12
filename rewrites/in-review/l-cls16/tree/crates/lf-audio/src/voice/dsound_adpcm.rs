//! The ADPCM DirectSound voice: ADPCM synthesis onto a sound device.
//!
//! Lifted from the verified rewrites of `rage::audVoiceDSoundAdpcm`. Its
//! position entry is the DirectSound voice's one word further into the
//! object; its queueing entry runs the ADPCM synth path with codec and
//! predictor tables; its seek entry maps the position through the
//! resampling helper or the milli-rate scaling. Collaborators are reached
//! through [`AdpcmWorld`].

use lf_core::Handle32;

use super::shared;
use super::{CodecTag, DeviceTag};

/// One ADPCM sample lane: the 14 copied parameter words, the synth
/// accumulator, and the source remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AdpcmLane {
    /// The 14 parameter words copied from the sample source.
    pub params: [u32; 14],
    /// The synth accumulator (zero when the synth branch does not run).
    pub acc: u32,
    /// The source bias word minus [`AdpcmLane::acc`].
    pub rem: u32,
}

/// What the ADPCM DirectSound voice needs from the engine and the sound
/// device: its state gate and converters, the ADPCM codec, and the
/// channel roles (cursor, region refill, gain).
pub trait AdpcmWorld {
    /// The voice's own state gate; only the low byte of the answer is
    /// tested.
    fn state_gate(&mut self) -> u32;
    /// Asks the device for its play cursor.
    fn play_cursor(&mut self, device: Option<Handle32<DeviceTag>>) -> u32;
    /// Converts a combined cursor against the voice rate; answers the
    /// playback position.
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32;
    /// Resolves a synth argument through the codec to a table entry.
    fn resolve_codec(&mut self, codec: Option<Handle32<CodecTag>>, arg: u32) -> u32;
    /// Converts a rate word against the voice rate.
    fn convert_rate(&mut self, word: u32, base: u32) -> u32;
    /// Resolves a seek position against the voice rate to a length.
    fn resolve_length(&mut self, pos: u32, rate: u32) -> u32;
    /// Refills the loop region with an explicit mode.
    fn refill_region(&mut self, mode: u32);
    /// Publishes the play cursor to the channel object.
    fn channel_set_cursor(&mut self, device: Option<Handle32<DeviceTag>>, cursor: u32);
    /// Forwards the voice gain.
    fn forward_gain(&mut self, gain: u32);
}

/// An ADPCM DirectSound voice, owning its header words, its sample
/// lanes, and the ADPCM codec and predictor tables its queueing entry
/// reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdpcmVoice {
    /// The flag byte.
    pub flags: u8,
    /// The sample-rate word.
    pub rate: u32,
    /// The ADPCM codec the queueing method resolves through.
    pub codec: Option<Handle32<CodecTag>>,
    /// The device object the seek publishes to and the position polls.
    pub device: Option<Handle32<DeviceTag>>,
    /// The level (gain) word read through the parameter block.
    pub level: u32,
    /// The loop base length.
    pub base_len: u32,
    /// The loop limit word.
    pub limit: u32,
    /// The loop combination word.
    pub combine: u32,
    /// The stored converted rate, also the bias the position adds.
    pub stored_rate: u32,
    /// The frequency word the position method scales.
    pub freq: u32,
    /// The ring cursor: index of the current lane.
    pub cursor: u32,
    /// The ring divisor; the cursor advances modulo this.
    pub divisor: u32,
    /// The ring of ADPCM sample lanes.
    pub lanes: Vec<AdpcmLane>,
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

impl AdpcmVoice {
    /// True when the current lane's remainder is zero.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last lane (the original reads
    /// whatever word lies there).
    pub fn lane_rest_quiet(&self) -> bool {
        self.lanes[self.cursor as usize].rem == 0
    }

    /// The current playback position, or all-ones when the state gate
    /// reports the voice is not playing. Otherwise the device answers
    /// its play cursor and the cursor combines with the scaled
    /// frequency word and the stored bias, converted against the voice
    /// rate.
    pub fn position<W: AdpcmWorld>(&mut self, world: &mut W) -> u32 {
        if world.state_gate() as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let pos = world.play_cursor(self.device);
        let scaled = shared::position_arg(self.freq, pos, self.stored_rate);
        world.convert_position(scaled, self.rate)
    }

    /// Queues one ADPCM synthesis block into the current lane and
    /// advances the cursor, like the PC ADPCM voice.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last lane, the divisor is zero,
    /// or the resolved table entries fall outside the owned tables (the
    /// original reads whatever lies at those addresses).
    pub fn queue_block<W: AdpcmWorld>(
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
            self.stored_rate = ans2;
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

    /// Seeks the voice to a sample. Looping voices with a nonzero base
    /// map the position through the resampling helper (doubled past the
    /// limit, combined below it); plain voices scale it by the rate in
    /// float and truncate. The loop region refills on the synth path,
    /// then the cursor publishes to the channel, the gain forwards, and
    /// the done bit sets.
    pub fn seek<W: AdpcmWorld>(&mut self, world: &mut W, pos: u32) {
        let cursor = if self.flags & shared::FLAG_LOOP != 0 && self.base_len != 0 {
            if pos < self.limit {
                let r = world.resolve_length(pos, self.rate);
                self.combine
                    .wrapping_add(r.wrapping_mul(2))
                    .wrapping_sub(self.base_len)
            } else {
                let r = world.resolve_length(pos.wrapping_sub(self.limit), self.rate);
                r.wrapping_add(r)
            }
        } else {
            let mut x5 = shared::u32_to_f32_bias(self.rate) * shared::u32_to_f32_bias(pos);
            x5 *= shared::MILLI;
            let x1 = shared::round_down_magic_adpcm(x5);
            let q = shared::trunc_f32_to_i64_lo(x1);
            // The original keeps only the low half of its truncating
            // store, then doubles it.
            q.wrapping_add(q)
        };
        if self.flags & shared::FLAG_SYNTH != 0 {
            world.refill_region(1);
        }
        world.channel_set_cursor(self.device, cursor);
        world.forward_gain(self.level);
        self.flags |= shared::FLAG_RESUME;
    }
}
