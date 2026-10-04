// original: 0x008a1c90 aud_sound_probe_voices
//! Probe the four voices of an audio sound, reporting their joint state.
//!
//! Each assigned voice is bound, then probed with the two stack arguments
//! and a mode bit taken from the sound's flags. A probe answering 2 ends
//! the scan immediately with 2; otherwise the result is 0 if any probe
//! answered 0 and 1 when every probe answered anything else (1 when no
//! voice is assigned at all).
//!
//! Callee ids (see contract): 1 = voice bind (thiscall/2), 2 = voice probe
//! (thiscall/3, answer drives the result).
//!
//! Note: the voice-table constants and `voice_for_slot` below are shared
//! verbatim with this lane's other rewrite files; keep one copy when merging.

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

export!(thiscall, rw_008a1c90(this: *mut u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let bank = *(this.add(0x40) as *const u8);
        let tune = *(this.add(0x54) as *const u32);
        let flag = ((*(this.add(0x39) as *const u8) >> 5) & 1) as u32;
        let mut ok = 1u32;
        for slot in 0..4usize {
            let v = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            if v == 0 {
                continue;
            }
            callee_thiscall!(1, u32, v, tune, 0);
            let v2 = voice_for_slot(bank, *(this.add(0x48 + slot) as *const u8));
            let r = callee_thiscall!(2, u32, v2, arg0, flag, arg1);
            if r == 2 {
                return 2;
            }
            if r == 0 {
                ok = 0;
            }
        }
        ok
    }
});
