// original: 0x0097DD20 audio_voice_route_submit (proposed)
//
// Resolve a voice handle for a channel and submit it. Looks up the channel
// record through the channel table, picks one of three record fields by the
// flag byte and the audio guards, optionally re-resolves through a scratch
// descriptor, then submits the voice with its level, pan and mode flag.
// Guards: the two field-selection blocks use the audio master flag, the
// generation pair and the mode word to choose between the record's fields.
// The mode flag submitted is 0x258 when bit 1 of the voice class word is
// clear, else 0. Nothing is submitted when the final handle is null.
//
// Original: 0x0097DD20 (thiscall, five stack args, void).

use lf_checker_rt::{callee_thiscall, relocated};

const MASTER_FLAG: u32 = 0x11F7060;
const GEN_A: u32 = 0x12088B4;
const GEN_B: u32 = 0x0F1C040;
const MODE_WORD: u32 = 0x1037720;
const MODE_SKIP: u32 = 0x12;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { (relocated(va) as *const u32).read_unaligned() }
}

const MODE_SELECT: u32 = 0x11F70CC;
const CHAN_TABLE: u32 = 0x115D9A0;
const VOICE_SELF: u32 = 0x1231310;
const CLASS_TABLE: u32 = 0x1295CD8;

const OBJ_OPTIONS: u32 = 8;
const OBJ_VOICE: u32 = 0x120;
const CLASS_INDEX_OFF: u32 = 0x2E;
const CLASS_WORD_OFF: u32 = 0x120;
const CLASS_BIT: u32 = 2;
const MODE_FLAG_SET: u32 = 0x258;

const CAL_CHAN_LOOKUP: u32 = 1;
const CAL_DESC_INIT: u32 = 2;
const CAL_DESC_FILL: u32 = 3;
const CAL_DESC_COMMIT: u32 = 4;
const CAL_SUBMIT_INIT: u32 = 5;
const CAL_SUBMIT: u32 = 6;

// File VA of the descriptor tag (a relocated image pointer).
const DESC_TAG: u32 = 0xE83134;

/// True when the audio guards pass (master flag set, generations match and
/// the mode word differs from the skip value), mirroring the original's
/// three-test sequence.
#[inline(always)]
unsafe fn guards_pass() -> bool {
    unsafe {
        if g32(MASTER_FLAG) == 1 {
            return false;
        }
        if g32(GEN_A) != g32(GEN_B) {
            return false;
        }
        g32(MODE_WORD) != MODE_SKIP
    }
}

lf_checker_rt::export!(thiscall, rw_0097DD20(obj: u32, chan: u32, flags: u32, pan: u32, level_bits: u32, gate: u32) -> u32 {
    unsafe {
        let record =
            callee_thiscall!(CAL_CHAN_LOOKUP, u32, relocated(CHAN_TABLE), chan);
        let flag = (flags & 0xFF) as u8;
        let mut voice = 0u32;
        if record != 0 {
            if flag == 0 {
                voice = rd32(record.wrapping_add(0x0E));
            } else {
                let sel = g32(MODE_SELECT);
                let use_alt = if guards_pass() { sel == 3 } else { sel == 4 || sel == 3 };
                voice = rd32(record.wrapping_add(if use_alt { 0x1A } else { 0x12 }));
            }
        }
        if rd32(obj.wrapping_add(OBJ_VOICE)) != 0
            && (voice == 0 || voice == g32(VOICE_SELF) || gate != 0)
        {
            let mut desc = [0u32; 5];
            callee_thiscall!(CAL_DESC_INIT, u32, desc.as_mut_ptr() as u32);
            callee_thiscall!(CAL_DESC_FILL, u32, desc.as_mut_ptr() as u32, gate);
            let resolved = desc[4];
            if resolved != 0 {
                if flag == 0 {
                    voice = rd32(resolved.wrapping_add(0x1A));
                } else {
                    let sel = g32(MODE_SELECT);
                    let use_alt =
                        if guards_pass() { sel == 3 } else { sel == 4 || sel == 3 };
                    voice = rd32(resolved.wrapping_add(if use_alt { 0x47 } else { 0x1E }));
                }
            }
            desc[0] = relocated(DESC_TAG);
            callee_thiscall!(CAL_DESC_COMMIT, u32, desc.as_mut_ptr() as u32);
        }
        if voice == 0 {
            return 0;
        }
        let mut frame = [0u32; 9];
        callee_thiscall!(CAL_SUBMIT_INIT, u32, frame.as_mut_ptr() as u32);
        frame[0] = level_bits;
        frame[5] = pan;
        frame[8] = rd32(obj.wrapping_add(OBJ_OPTIONS));
        let class_ptr = rd32(obj.wrapping_add(OBJ_VOICE));
        let index = unsafe {
            (class_ptr.wrapping_add(CLASS_INDEX_OFF) as *const u16).read_unaligned() as i16 as i32
        };
        let entry = rd32(
            relocated(CLASS_TABLE).wrapping_add((index as u32).wrapping_mul(4)),
        );
        let class_word = rd32(entry.wrapping_add(CLASS_WORD_OFF));
        frame[1] = if class_word & CLASS_BIT == 0 { MODE_FLAG_SET } else { 0 };
        callee_thiscall!(
            CAL_SUBMIT, u32, obj,
            voice,
            frame.as_ptr() as u32,
            0xFFFF_FFFFu32, 0, 0
        );
        0
    }
});
