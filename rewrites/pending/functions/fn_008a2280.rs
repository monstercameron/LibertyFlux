// original: 0x008a2280 rage::audCollapsingStereoSound::vf7
//! Collapse a stereo sound's four channels onto solved voices.
//!
//! After the gate accepts the request, each of the four channels is solved
//! in turn: the mix cursor is preserved across the solver call, and the
//! returned voice address is converted back to a slot value by subtracting
//! the bank base and dividing by the voice stride (an unassigned channel
//! keeps the empty marker). If any channel ends up without a voice the
//! collapse is rejected. Otherwise each parameter set is routed through
//! the melt hook, the first pair is retuned (from live parameters when
//! both are present, else with fixed defaults), the block's levels are
//! latched, and the collapse is accepted. Returns 1 on accept, 0 on reject.
//!
//! Callee ids (see contract): 1 = collapse gate (thiscall/3, low byte
//! tested), 2 = field solver (thiscall/4 on the global field object, answer
//! converted back to a slot), 3 = melt hook (thiscall/1 through the
//! object's function table slot 4), 4 = retune (thiscall/1).
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

export!(thiscall, rw_008a2280(this: *mut u8, arg0: u32, arg1: u32, cursor: *mut u8) -> u32 {
    unsafe {
        let gate: u32 = callee_thiscall!(1, u32, this as u32, arg0, arg1, cursor as u32);
        if gate & 0xFF == 0 {
            return 0;
        }
        let bank = *(this.add(0x40) as *const u8);
        let blk = *(this.add(0x94) as *const u32);
        let stride = *(global::<u32>(G_VOICE_STRIDE) as *const u32);
        let table = *(global::<u32>(G_VOICE_TABLE) as *const u32);
        let row = (bank as u32).wrapping_mul(VOICE_BANK_STRIDE);
        let tab = *((table.wrapping_add(row).wrapping_add(VOICE_TABLE_BIAS)) as *const u32);
        let field = relocated(0x115dc18);
        for pass in 0..4u32 {
            let mut saved = [0u32; 6];
            for i in 0..6 {
                saved[i] = *((cursor as *const u32).add(i));
            }
            let pick = *((blk.wrapping_add(if pass % 2 == 0 { 0 } else { 4 })) as *const u32);
            let ans: u32 = callee_thiscall!(2, u32, field, pick, this as u32, arg1, cursor as u32);
            for i in 0..6 {
                *((cursor as *mut u32).add(i)) = saved[i];
            }
            // A null answer leaves the channel empty; otherwise the slot is
            // the voice index the solver address encodes. The stride is a
            // fixed table layout constant and is never zero in practice.
            let slot = if ans == 0 {
                NO_VOICE
            } else if stride == 0 {
                NO_VOICE
            } else {
                (ans.wrapping_sub(tab) / stride) as u8
            };
            *(this.add((0x48 + pass) as usize) as *mut u8) = slot;
        }
        for slot in 0..4usize {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v == 0 {
                return 0;
            }
        }
        let vtable = *(this as *const u32);
        let melt: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(0x10)) as *const u32) as usize);
        for i in 0..5u32 {
            let p = *((blk.wrapping_add(0x10).wrapping_add(i.wrapping_mul(4))) as *const u32);
            if p != 0 {
                let ans = melt(this as u32, p);
                *((this.add(0xb8).wrapping_add((i.wrapping_mul(4)) as usize)) as *mut u32) = ans;
            }
        }
        let tune0 = *(this.add(0xc4) as *const u32);
        let tune1 = *(this.add(0xc8) as *const u32);
        if tune1 != 0 && tune0 != 0 {
            let a = cvt_f32_i32(f32::from_bits(*(tune0 as *const u32)));
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(4, u32, v0, a as u32);
            let b = cvt_f32_i32(f32::from_bits(*(tune1 as *const u32)));
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(4, u32, v1, b as u32);
        } else {
            let v0 = voice_for_slot(bank, *(this.add(0x48) as *const u8));
            callee_thiscall!(4, u32, v0, 0x10e);
            let v1 = voice_for_slot(bank, *(this.add(0x49) as *const u8));
            callee_thiscall!(4, u32, v1, 0x5a);
        }
        *(this.add(0xb0) as *mut u32) = *((blk.wrapping_add(8)) as *const u32);
        *(this.add(0xb4) as *mut u32) = *((blk.wrapping_add(0xc)) as *const u32);
        *(this.add(0xcd) as *mut u8) = *((blk.wrapping_add(0x24)) as *const u8);
        1
    }
});
