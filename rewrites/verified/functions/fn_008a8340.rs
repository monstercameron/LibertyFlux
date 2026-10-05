// original: 0x008a8340 audio_voice_gain_normalize
//
// Normalize up to six voice channel gains.
//
// `obj` holds a flag byte at +0xD2 and receives six output floats at
// +0x0..+0x14; `count` (0-6) says how many channels are live. Each live
// channel gets 1.0 when bit `(flag >> table[i]) & 1` is set, else 0.0.
// When `count` is 2 and flag bit 0x20 is set, both channels use the
// global float instead and the live count is forced to 1. Every live
// channel is divided by `sqrt(float(double(live) + C0))`; dead channels
// read 0. Slots the loop never writes read the stack fill (0 by
// contract). Returns 0. Stdcall.

use lf_checker_rt::{export, global};

/// Base of the byte table of shift counts (one byte read per channel at
/// stride 4).
const GAIN_SHIFT_TABLE: u32 = 0xe7b9fc;
/// Global float overriding both channels when count == 2 and the stereo
/// flag is set.
const GAIN_STEREO_VALUE: u32 = 0xfe8830;
/// Global double added to the live count before the sqrt.
const GAIN_COUNT_BASE: u32 = 0xfe8f50;
/// Flag byte offset in the voice object.
const GAIN_FLAG_OFF: u32 = 0xd2;
/// Flag bit selecting the stereo override at count == 2.
const GAIN_STEREO_BIT: u8 = 0x20;
/// Output channels filled by this function.
const GAIN_CHANNELS: usize = 6;

#[inline(always)]
unsafe fn rf32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read_unaligned() }
}
#[inline(always)]
unsafe fn wf32(addr: u32, v: f32) {
    unsafe { (addr as *mut f32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn wu32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

export!(stdcall, rw_008a8340(obj: u32, count: u32) -> u32 {
    unsafe {
        let flags = (obj.wrapping_add(GAIN_FLAG_OFF) as *const u8).read();
        let mut tmp = [0f32; GAIN_CHANNELS];
        let mut live = 0u32;
        let mut i = 0u32;
        while i < count {
            let shift = global::<u8>(GAIN_SHIFT_TABLE)
                .wrapping_add(i.wrapping_mul(4) as usize)
                .read();
            // x86 `(an instruction of the original)` masks the count to 5 bits; counts of 8 or
            // more shift every bit out of the byte.
            let n = shift & 31;
            let bit = if n >= 8 { 0 } else { (flags >> n) & 1 };
            if bit == 1 {
                tmp[i as usize] = 1.0;
                live += 1;
            }
            i += 1;
        }
        let (ch0, ch1);
        if count == 2 && flags & GAIN_STEREO_BIT != 0 {
            let g = rf32(global::<u32>(GAIN_STEREO_VALUE) as u32);
            ch0 = g;
            ch1 = g;
            live = 1;
        } else {
            ch0 = tmp[0];
            ch1 = tmp[1];
        }
        let c0 = (global::<u64>(GAIN_COUNT_BASE) as *const f64).read_unaligned();
        let norm = ((live as f64 + c0) as f32).sqrt();
        if live != 0 && count != 0 {
            wf32(obj, ch0 / norm);
        } else {
            wu32(obj, 0);
        }
        if live != 0 && count > 1 {
            wf32(obj.wrapping_add(4), ch1 / norm);
        } else {
            wu32(obj.wrapping_add(4), 0);
        }
        if live != 0 && count > 2 {
            wf32(obj.wrapping_add(8), tmp[2] / norm);
        } else {
            wu32(obj.wrapping_add(8), 0);
        }
        if live != 0 && count > 3 {
            wf32(obj.wrapping_add(0xc), tmp[3] / norm);
        } else {
            wu32(obj.wrapping_add(0xc), 0);
        }
        if live != 0 && count > 4 {
            wf32(obj.wrapping_add(0x10), tmp[4] / norm);
        } else {
            wu32(obj.wrapping_add(0x10), 0);
        }
        if live != 0 && count > 5 {
            wf32(obj.wrapping_add(0x14), tmp[5] / norm);
        } else {
            wu32(obj.wrapping_add(0x14), 0);
        }
        0
    }
});
