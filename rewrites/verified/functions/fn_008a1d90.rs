// original: 0x008a1d90 aud_sound_update_distance
//! Recompute a sound's distance factor, re-level its voices, settle them.
//!
//! When the sound is live, derives an attenuation factor (clamped live value,
//! or a smoothstep of the solved listener distance between the near and far
//! bounds), maps it through a curve per channel pair, and pushes the levels
//! to the assigned voices. The first pair is retuned from live parameters
//! when present. Finally each voice whose state word reads settled is asked
//! to settle with the stack argument; the result is 1 if any of them
//! confirmed, else 0.
//!
//! Callee ids (see contract): 1 = offset solver (thiscall/3, out-pointer +
//! in-pointer frame args, answered with a 3-vector), 2/3 = curve lookups for
//! the two channel pairs (cdecl/1, f32 in ST0), 4 = level set (thiscall/1),
//! 5 = retune (thiscall/1), 6 = voice settle (thiscall/1).
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

export!(thiscall, rw_008a1d90(this: *mut u8, arg0: u32) -> u32 {
    unsafe {
        let bank = *(this.add(0x40) as *const u8);
        let base_level = f32::from_bits(*(this.add(0xb0) as *const u32));
        // Entry gate: a non-negative base level, a far bound, or a live
        // value each admit the main path; otherwise only the tail runs.
        // Written with negated comparisons so unordered (NaN) inputs take
        // the same branches as the original's comiss/jae/jb/jbe tests.
        let gate1 = !(0.0 > base_level)
            || *(this.add(0xbc) as *const u32) != 0
            || *(this.add(0xc0) as *const u32) != 0;
        if gate1 && *(this.add(0xcd) as *const u8) != 0 {
            let live = *(this.add(0xc0) as *const u32);
            let f: f32 = if live != 0 {
                let d = f32::from_bits(*(live as *const u32));
                if 0.0 >= d {
                    0.0
                } else if d >= 1.0 {
                    1.0
                } else {
                    d
                }
            } else {
                let tag = *(this.add(0xcc) as *const u8);
                let stride2 = *(global::<u32>(0x115d968) as *const u32);
                let table = *(global::<u32>(G_VOICE_TABLE) as *const u32);
                let row = (bank as u32).wrapping_mul(VOICE_BANK_STRIDE);
                let base = *((table.wrapping_add(row).wrapping_add(0x6f14)) as *const u32);
                let st = base.wrapping_add((tag as u32).wrapping_mul(stride2));
                let sel = ((*(st.wrapping_add(0xe7) as *const u8)) & 7) as u32;
                let pick = *(global::<u32>(0x115f80c + sel.wrapping_mul(8)) as *const u32);
                let k = pick.wrapping_add(1).wrapping_mul(2);
                let inp = [
                    *((st.wrapping_add(k.wrapping_mul(8))) as *const u32),
                    *((st.wrapping_add(k.wrapping_mul(8)).wrapping_add(4)) as *const u32),
                    *((st.wrapping_add(k.wrapping_mul(8)).wrapping_add(8)) as *const u32),
                ];
                let obj = *(global::<u32>(0x115f7f4) as *const u32);
                let mut inp = inp;
                let mut out = [0u32; 3];
                callee_thiscall!(1, u32, obj, out.as_mut_ptr() as u32, inp.as_mut_ptr() as u32, 0);
                let x = f32::from_bits(out[0]);
                let y = f32::from_bits(out[1]);
                let z = f32::from_bits(out[2]);
                let len = ((x * x + y * y) + z * z).sqrt();
                let near_at = *(this.add(0xb8) as *const u32);
                let near = if near_at != 0 {
                    f32::from_bits(*(near_at as *const u32))
                } else {
                    base_level
                };
                let far_at = *(this.add(0xbc) as *const u32);
                let far = if far_at != 0 {
                    f32::from_bits(*(far_at as *const u32))
                } else {
                    f32::from_bits(*(this.add(0xb4) as *const u32))
                };
                if !(near >= len) {
                    if !(len >= far) {
                        (len - near) / (far - near)
                    } else {
                        1.0
                    }
                } else {
                    0.0
                }
            };
            let g1: f32 = callee_cdecl!(2, f32, (1.0 - f).to_bits());
            let g2: f32 = callee_cdecl!(3, f32, f.to_bits());
            let levels = [g1.to_bits(), g1.to_bits(), g2.to_bits(), g2.to_bits()];
            for (slot, level) in levels.iter().enumerate() {
                let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
                if v != 0 {
                    callee_thiscall!(4, u32, v, *level);
                }
            }
        }
        let par0 = *(this.add(0xc4) as *const u32);
        let par1 = *(this.add(0xc8) as *const u32);
        if par1 != 0 && par0 != 0 {
            let a = cvt_f32_i32(f32::from_bits(*(par0 as *const u32)));
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(5, u32, v0, a as u32);
            let b = cvt_f32_i32(f32::from_bits(*(par1 as *const u32)));
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(5, u32, v1, b as u32);
        }
        let mut settled = 0u8;
        for slot in 0..4usize {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v == 0 {
                continue;
            }
            if *(v.wrapping_add(6) as *const u16) != 2 {
                continue;
            }
            let v3 = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            let r: u32 = callee_thiscall!(6, u32, v3, arg0);
            if r & 0xFF != 0 {
                settled = 1;
            }
        }
        settled as u32
    }
});
