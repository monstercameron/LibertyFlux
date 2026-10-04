// original: 0x008a1510 aud_sound_refresh_channels
//! Update the two stereo channels of an audio sound.
//!
//! For each channel, when a voice is assigned and its gate flag is clear the
//! voice is started and the channel retriggered; when no voice is assigned
//! but the gate flag is set, the fallback trigger path runs instead. Both
//! channels then get a level update carrying this frame's mix value.
//! Original is a thiscall taking the sound in ECX and a mix block on the
//! stack; it returns nothing meaningful.
//!
//! Callee ids (see contract): 1 = voice start/stop helper (thiscall/0 on the
//! voice), 2 = channel retrigger (thiscall/1), 3 = fallback trigger
//! (thiscall/3), 4 = level update (thiscall/1, float bits).
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

export!(thiscall, rw_008a1510(this: *mut u8, mix: *mut u8) -> u32 {
    unsafe {
        let bank = *(this.add(0x40) as *const u8);
        for ch in 0..2u32 {
            let idx = *(this.add(0x48 + ch as usize) as *const u8);
            let v = voice_for_slot(bank, idx);
            let gate = *(mix.add((0x73 + ch) as usize) as *const u8);
            if v != 0 {
                if gate == 0 {
                    callee_thiscall!(1, u32, v);
                    callee_thiscall!(2, u32, this as u32, ch);
                }
            } else if gate != 0 {
                let param = *(mix.add((0x64 + ch * 4) as usize) as *const u32);
                let anchor = (mix as u32).wrapping_add(0x28);
                callee_thiscall!(3, u32, this as u32, param, ch, anchor);
            }
        }
        for ch in 0..2u32 {
            let idx = *(this.add(0x48 + ch as usize) as *const u8);
            let v = voice_for_slot(bank, idx);
            if v != 0 {
                let level_bits = *(mix.add((0x40 + ch * 4) as usize) as *const u32);
                callee_thiscall!(4, u32, v, level_bits);
            }
        }
        0
    }
});
