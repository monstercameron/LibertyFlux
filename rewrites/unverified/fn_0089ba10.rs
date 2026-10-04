// original: 0x0089BA10 audio_dual_voice_probe
//! Dual voice probe: looks up two voice slots from the audio route table,
//! forwards a stored parameter through a setter, then polls both voices and
//! merges the two poll results (2 dominates, then 0, else 1).
//!
//! Table walk per voice: `row = table_base + route*0x6F40 + 0x6F10`,
//! `voice = stride*slot + *row`; slot 0xFF means "no voice" (null).

use lf_k2_rt::{callee_thiscall, export, global};

const ROUTE_STRIDE: u32 = 0x6F40;
const ROW_HDR: u32 = 0x6F10;
const NO_VOICE: u8 = 0xFF;

fn voice_ptr(stride: u32, table: u32, route: u8, slot: u8) -> u32 {
    if slot == NO_VOICE {
        return 0;
    }
    let row = table
        .wrapping_add((route as u32).wrapping_mul(ROUTE_STRIDE))
        .wrapping_add(ROW_HDR);
    stride
        .wrapping_mul(slot as u32)
        .wrapping_add(unsafe { *(row as *const u32) })
}

export!(thiscall, rw_0089BA10(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let t = this as *const u8;
        let route = *t.add(0x40);
        let slot_a = *t.add(0x48);
        let slot_b = *t.add(0x49);
        let flag = (*t.add(0x39) >> 5) & 1;
        let stored = *(this as *const u32).add(0x54 / 4);
        let stride = *global::<u32>(0x115D964);
        let table = *global::<u32>(0x115D988);

        let va = voice_ptr(stride, table, route, slot_a);
        // Setter takes (stored_param, 0); result ignored.
        callee_thiscall!(1, u32, va, stored, 0);

        // The original stages the one-byte flag in its pushed-ECX stack slot
        // and forwards the whole word, so the upper three bytes are the entry
        // ECX (this) with the flag in the low byte. Entry ECX is a declared
        // checker input, identical on both sides, so this word is reproduced
        // exactly rather than skipped: skipping would blind the flag bit.
        let flag_word = (this & 0xFFFF_FF00) | flag as u32;
        let ra = callee_thiscall!(2, u32, va, arg0, flag_word, arg1);
        let vb = voice_ptr(stride, table, route, slot_b);
        let rb = callee_thiscall!(3, u32, vb, arg0, flag_word, arg1);
        if ra == 2 || rb == 2 {
            2
        } else if ra == 0 || rb == 0 {
            0
        } else {
            1
        }
    }
});
