//! The DirectSound voice: a voice played through a sound device.
//!
//! Lifted from the verified rewrites of `rage::audVoiceDSound`. The 32-bit
//! object is a header (flag byte, rate word, two device objects, loop
//! words, frequency words) followed by a ring of sample windows; here
//! those are plain fields, with device objects carried as opaque cookies
//! and reached through [`DSoundWorld`]. The device roles are operating
//! system calls; they stay behind this trait until the platform audio
//! backend lands (see the module docs for the mapping).

use lf_core::Handle32;

use super::DeviceTag;
use super::shared;

/// One sample window of the DirectSound voice's ring: the 14 copied
/// source words, the derived doubled count, and the source remainder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DSoundLane {
    /// The 14 source words copied into the window.
    pub window: [u32; 14],
    /// Twice the scaled sample count.
    pub even: u32,
    /// The source bias word minus [`DSoundLane::even`].
    pub rest: u32,
}

/// What the DirectSound voice needs from the engine and the sound
/// device: its state gate and converters, its cached voice resolution,
/// its channel notification, and the device roles (cursor, resume,
/// seek, length, release).
///
/// Every method is one callee or virtual-slot role from the verified
/// rewrites, with addresses narrowed to opaque cookies, constant zero
/// words dropped, and unused answers unmodelled.
pub trait DSoundWorld {
    /// The voice's own state gate; only the low byte of the answer is
    /// tested.
    fn state_gate(&mut self) -> u32;
    /// Asks the device for its play cursor.
    fn play_cursor(&mut self, device: Option<Handle32<DeviceTag>>) -> u32;
    /// Converts a combined cursor against the voice rate; answers the
    /// playback position.
    fn convert_position(&mut self, scaled: u32, base: u32) -> u32;
    /// Resolves the incoming sample count against the voice rate to the
    /// cached voice word.
    fn resolve_voice(&mut self, count: u32, rate: u32) -> u32;
    /// Consumes stream samples; the voice keeps the count minus a
    /// positive answer.
    fn consume_stream(&mut self, count: u32) -> u32;
    /// Resolves a seek position against the voice rate to a length.
    fn resolve_length(&mut self, pos: u32, rate: u32) -> u32;
    /// Runs the voice's own start notification with an explicit mode.
    fn notify_start(&mut self, mode: u32);
    /// Delivers a loop length to the channel object.
    fn channel_set_length(&mut self, device: Option<Handle32<DeviceTag>>, len: u32);
    /// Runs the voice's finish entry with the level word; answers its
    /// answer.
    fn finish_start(&mut self, level: u32) -> u32;
    /// Releases a device object.
    fn release_device(&mut self, device: Option<Handle32<DeviceTag>>);
    /// The base teardown entry; answers its answer.
    fn base_teardown(&mut self) -> u32;
    /// Tells the device to resume, with the loop flag.
    fn device_resume(&mut self, device: Option<Handle32<DeviceTag>>, looping: bool);
    /// Hands the voice's restart position to the device.
    fn device_seek(&mut self, device: Option<Handle32<DeviceTag>>, pos: u32);
}

/// A DirectSound voice, owning its header words and sample windows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DSoundVoice {
    /// The flag byte.
    pub flags: u8,
    /// The sample-rate word.
    pub rate: u32,
    /// The restart position word; its low bit also feeds the stopping
    /// check.
    pub restart_pos: u32,
    /// The status byte read through the parameter block.
    pub params_status: u8,
    /// The primary device object.
    pub device_a: Option<Handle32<DeviceTag>>,
    /// The secondary device object.
    pub device_b: Option<Handle32<DeviceTag>>,
    /// The cached voice word the queueing method stores, also the bias
    /// the position method adds.
    pub cached_voice: u32,
    /// The frequency word the position method scales.
    pub freq: u32,
    /// The loop base length.
    pub base_len: u32,
    /// The loop limit word.
    pub limit: u32,
    /// The loop combination word.
    pub combine: u32,
    /// The level word read through the parameter block.
    pub level: u32,
    /// The ring cursor: index of the current window.
    pub cursor: u32,
    /// The ring divisor; the cursor advances modulo this.
    pub divisor: u32,
    /// The ring of sample windows.
    pub lanes: Vec<DSoundLane>,
}

impl DSoundVoice {
    /// True when any of three status flags is set: the stopped or
    /// resume-pending bits of the flag byte, the low bit of the restart
    /// position, or the stopping bit of the parameter status byte.
    pub fn is_stopping(&self) -> bool {
        if self.flags & (shared::FLAG_STOPPED | shared::FLAG_RESUME) != 0 {
            return true;
        }
        if self.restart_pos & 1 != 0 {
            return true;
        }
        self.params_status & 0x40 != 0
    }

    /// True when the current window's remainder is zero.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last window (the original reads
    /// whatever word lies there).
    pub fn slot_rest_quiet(&self) -> bool {
        self.lanes[self.cursor as usize].rest == 0
    }

    /// Resumes the voice on its device, unless resume is not pending or
    /// the voice is stopped. Otherwise resumes the device with the loop
    /// flag (set when the loop or synth bits show), seeks it to the
    /// restart position, and clears the pending bit.
    pub fn resume<W: DSoundWorld>(&mut self, world: &mut W) {
        if self.flags & shared::FLAG_RESUME == 0 || self.flags & shared::FLAG_STOPPED != 0 {
            return;
        }
        let looping = self.flags & (shared::FLAG_LOOP | shared::FLAG_SYNTH) != 0;
        world.device_resume(self.device_a, looping);
        world.device_seek(self.device_a, self.restart_pos);
        self.flags &= !shared::FLAG_RESUME;
    }

    /// The current playback position, or all-ones when the state gate
    /// reports the voice is not playing. Otherwise the device answers
    /// its play cursor and the cursor combines with the scaled
    /// frequency word and the cached bias, converted against the voice
    /// rate.
    pub fn position<W: DSoundWorld>(&mut self, world: &mut W) -> u32 {
        if world.state_gate() as u8 == 0 {
            return 0xFFFF_FFFF;
        }
        let pos = world.play_cursor(self.device_a);
        let scaled = shared::position_arg(self.freq, pos, self.cached_voice);
        world.convert_position(scaled, self.rate)
    }

    /// Refreshes the current sample window and advances the cursor,
    /// answering the division quotient of the step. Copies the 14
    /// source words into the window, optionally resolves the cached
    /// voice and consumes stream samples, derives the doubled scaled
    /// count and the remainder, and steps the cursor modulo the
    /// divisor.
    ///
    /// # Panics
    ///
    /// When the cursor selects past the last window, or the divisor is
    /// zero (the original divides there too).
    pub fn refresh_slot<W: DSoundWorld>(
        &mut self,
        world: &mut W,
        src: &[u32; 14],
        src_bias: u32,
        count: u32,
    ) -> u32 {
        let idx = self.cursor as usize;
        self.lanes[idx].window.copy_from_slice(src);
        let mut left = count;
        if count != 0 && self.flags & shared::FLAG_SYNTH != 0 {
            let voice = world.resolve_voice(left, self.rate);
            self.cached_voice = voice;
            let used = world.consume_stream(left);
            if (used as i32) > 0 {
                left = left.wrapping_sub(used);
            }
        }
        let even = shared::milli_floor_doubled(self.rate, left);
        self.lanes[idx].even = even;
        self.lanes[idx].rest = src_bias.wrapping_sub(even);
        let next = self.cursor.wrapping_add(1);
        self.cursor = next % self.divisor;
        next / self.divisor
    }

    /// Recomputes the loop length and notifies the channel, answering
    /// the finish entry's answer. On the loop path with a nonzero base
    /// the length resolves through the engine (doubled past the limit,
    /// combined below it); otherwise it is the doubled scaled seek
    /// position. The start notification runs first on the synth path,
    /// then the length delivers to the channel, the finish entry runs
    /// with the level word, and the done bit sets.
    pub fn seek<W: DSoundWorld>(&mut self, world: &mut W, pos: u32) -> u32 {
        let len = if self.flags & shared::FLAG_LOOP != 0 && self.base_len != 0 {
            if pos >= self.limit {
                let v = world.resolve_length(pos.wrapping_sub(self.limit), self.rate);
                v.wrapping_add(v)
            } else {
                let v = world.resolve_length(pos, self.rate);
                self.combine
                    .wrapping_add(v.wrapping_mul(2))
                    .wrapping_sub(self.base_len)
            }
        } else {
            shared::milli_floor_doubled(self.rate, pos)
        };
        if self.flags & shared::FLAG_SYNTH != 0 {
            world.notify_start(1);
        }
        world.channel_set_length(self.device_a, len);
        let r = world.finish_start(self.level);
        self.flags |= shared::FLAG_RESUME;
        r
    }

    /// Tears the voice down: releases both present devices, then runs
    /// the base teardown and answers its answer.
    pub fn teardown<W: DSoundWorld>(&mut self, world: &mut W) -> u32 {
        if self.device_a.is_some() {
            world.release_device(self.device_a);
        }
        if self.device_b.is_some() {
            world.release_device(self.device_b);
        }
        world.base_teardown()
    }
}
