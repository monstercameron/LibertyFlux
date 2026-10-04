// original: 0x008a18e0 aud_sound_reset_and_retrigger
//! Reset pass and conditional retrigger for an audio sound.
//!
//! Tags the sound, parks all four voice levels (silence on the first pair,
//! a floor value on the second), retunes the first pair from the live
//! parameter blocks when present, then asks for an attenuation factor: a
//! factor of exactly one retriggers the first pair through the channel
//! lookup, a factor of exactly zero retriggers the second pair instead.
//! Every assigned voice is rebound and refreshed at the end, the refresh
//! carrying the stack argument through. Returns nothing meaningful.
//!
//! Callee ids (see contract): 1 = tag query (thiscall/0, low byte stored),
//! 2 = level set (thiscall/1, float bits), 3 = retune (thiscall/1, truncated
//! int), 4 = factor query (thiscall/1, f32 in ST0), 5 = channel lookup
//! (thiscall/1), 6 = voice start (thiscall/0), 7 = channel retrigger
//! (thiscall/1), 8 = voice bind (thiscall/2), 9 = voice refresh (thiscall/1).
//!
//! Note: the voice-table constants, `voice_for_slot` and `cvt_f32_i32` below
//! are shared verbatim with this lane's other rewrite files; keep one copy
//! when merging.

/// Row stride of the voice bank table: one bank holds voices for 256 slots.
const VOICE_BANK_STRIDE: u32 = 0x6f40;
/// Bias from a bank row start to its voice-array pointer.
const VOICE_TABLE_BIAS: u32 = 0x6f10;
/// Slot value meaning "no voice assigned".
const NO_VOICE: u8 = 0xFF;
/// Global: byte stride between adjacent voices in a voice array.
const G_VOICE_STRIDE: u32 = 0x115d964;
/// Global: pointer to the voice bank table.
const G_VOICE_TABLE: u32 = 0x115d988;

/// Resolve the voice object for bank `bank` and slot value `idx`.
///
/// A slot holding 0xFF means no voice; otherwise the bank table gives the
/// voice-array base and the slot value indexes into it with the global stride.
fn voice_for_slot(bank: u8, idx: u8) -> u32 {
    if idx == NO_VOICE {
        return 0;
    }
    unsafe {
        let stride = *(global::<u32>(G_VOICE_STRIDE) as *const u32);
        let table = *(global::<u32>(G_VOICE_TABLE) as *const u32);
        let row = (bank as u32).wrapping_mul(VOICE_BANK_STRIDE);
        let base = *((table.wrapping_add(row).wrapping_add(VOICE_TABLE_BIAS)) as *const u32);
        base.wrapping_add(stride.wrapping_mul(idx as u32))
    }
}

/// Truncate a float toward zero with x86 `cvttss2si` semantics: NaN and
/// out-of-range inputs yield 0x80000000 instead of saturating the way a
/// plain Rust float-to-int cast would.
fn cvt_f32_i32(f: f32) -> i32 {
    if f.is_nan() || f >= 2147483648.0 || f < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        f as i32
    }
}

export!(thiscall, rw_008a18e0(this: *mut u8, refresh_arg: u32) -> u32 {
    unsafe {
        let tag = callee_thiscall!(1, u32, this as u32);
        *(this.add(0xcc) as *mut u8) = (tag & 0xFF) as u8;
        let bank = *(this.add(0x40) as *const u8);
        let saved = *(this.add(0x54) as *const u32);
        const PARK_LEVELS: [u32; 4] = [0, 0, 0xC2C8_0000, 0xC2C8_0000];
        for (slot, level) in PARK_LEVELS.iter().enumerate() {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v != 0 {
                callee_thiscall!(2, u32, v, *level);
            }
        }
        let par0 = *(this.add(0xc4) as *const u32);
        let par1 = *(this.add(0xc8) as *const u32);
        if par1 != 0 && par0 != 0 {
            let a = cvt_f32_i32(f32::from_bits(*(par0 as *const u32)));
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(3, u32, v0, a as u32);
            let b = cvt_f32_i32(f32::from_bits(*(par1 as *const u32)));
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(3, u32, v1, b as u32);
        }
        let factor: f32 = callee_thiscall!(4, f32, this as u32, 1);
        if *(this.add(0xcd) as *const u8) == 0 {
            let full = f32::from_bits(*(global::<u32>(0xfe88e8) as *const u32));
            if factor == full {
                for ch in 0..2u32 {
                    let v = voice_for_slot(bank, *(this.add((0x48 + ch) as usize) as *const u8));
                    if v != 0 {
                        let t = callee_thiscall!(5, u32, this as u32, ch);
                        callee_thiscall!(6, u32, t);
                        callee_thiscall!(7, u32, this as u32, ch);
                    }
                }
            } else if factor == 0.0 {
                for ch in 2..4u32 {
                    let t = callee_thiscall!(5, u32, this as u32, ch);
                    if t != 0 {
                        let t2 = callee_thiscall!(5, u32, this as u32, ch);
                        callee_thiscall!(6, u32, t2);
                        callee_thiscall!(7, u32, this as u32, ch);
                    }
                }
            }
        }
        for slot in 0..4usize {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v != 0 {
                callee_thiscall!(8, u32, v, saved, 0);
                let v2 = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
                callee_thiscall!(9, u32, v2, refresh_arg);
            }
        }
        0
    }
});
