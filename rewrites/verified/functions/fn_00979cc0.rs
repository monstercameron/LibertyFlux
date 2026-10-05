// original: 0x00979CC0 audio_collision_queues_drain (proposed)
//
// Drain two fixed-stride queues owned by an audio entity, then clear both
// counts. Guards: returns immediately when the audio master flag is 1, when
// the two generation words differ, or when the mode word is 0x12. Queue one
// (count at +0xFE50, SIGNED, items of 0x20 bytes from +0xFA64) calls the slot
// initialiser and then the slot submitter for each item, passing the item's
// float, its header dword and a pointer to a six-word frame struct. Queue two
// (count at +0x10E60, SIGNED, items of 0x80 bytes from +0xFE8C) calls the item
// filler and the item finaliser per item, and sets bit 0x10 at +0xE8 of the
// voice struct reached through a two-level table for each of the item's two
// voice pointers. A voice whose class byte is 0xFF resolves to address zero
// and faults on the flag write; the rewrite faults identically.
//
// Original: 0x00979CC0 (thiscall, no stack args, void).
use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated};


const MASTER_FLAG: u32 = 0x11F7060;
const GEN_A: u32 = 0x12088B4;
const GEN_B: u32 = 0x0F1C040;
const MODE_WORD: u32 = 0x1037720;
const MODE_SKIP: u32 = 0x12;
const STRIDE_PTR: u32 = 0x115D968;
const TABLE_PTR: u32 = 0x115D988;

const Q1_COUNT: u32 = 0xFE50;
const Q1_BASE: u32 = 0xFA64;
const Q1_STRIDE: u32 = 0x20;
const Q2_COUNT: u32 = 0x10E60;
const Q2_BASE: u32 = 0xFE8C;
const Q2_STRIDE: u32 = 0x80;
const Q2_STRUCT_BACK: u32 = 0x2C;

const TABLE_ENTRY_STRIDE: u32 = 0x6F40;
const TABLE_BIAS: u32 = 0x6F14;
const VOICE_CLASS: u32 = 4;
const VOICE_INDEX: u32 = 0x40;
const VOICE_FLAG_OFF: u32 = 0xE8;
const VOICE_FLAG_BIT: u8 = 0x10;
const CLASS_FAULT: u8 = 0xFF;

const CAL_SLOT_INIT: u32 = 1;
const CAL_SLOT_SUBMIT: u32 = 2;
const CAL_ITEM_FILL: u32 = 3;
const CAL_ITEM_FINI: u32 = 4;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn g32(va: u32) -> u32 {
    unsafe { (relocated(va) as *const u32).read_unaligned() }
}

/// Set bit 0x10 at `base + 0xE8` with a volatile read-modify-write, exactly
/// like the original's `or byte` (including the fault when base is zero).
#[inline(always)]
unsafe fn set_voice_flag(base: u32) {
    unsafe {
        let p = (base.wrapping_add(VOICE_FLAG_OFF)) as *mut u8;
        let v = (p as *const u8).read_volatile();
        p.write_volatile(v | VOICE_FLAG_BIT);
    }
}

lf_checker_rt::export!(thiscall, rw_00979CC0(obj: u32) -> u32 {
    unsafe {
        if g32(MASTER_FLAG) == 1 {
            return 0;
        }
        if g32(GEN_A) != g32(GEN_B) {
            return 0;
        }
        if g32(MODE_WORD) == MODE_SKIP {
            return 0;
        }
        // Queue one: submit every pending slot (count is SIGNED).
        let mut i = 0i32;
        if (rd32(obj.wrapping_add(Q1_COUNT)) as i32) > 0 {
            let mut item = obj.wrapping_add(Q1_BASE);
            // One fixed frame slot reused every iteration, like the original.
            let mut frame = [0u32; 6];
            loop {
                callee_thiscall!(CAL_SLOT_INIT, u32, frame.as_mut_ptr() as u32);
                frame[0] = rd32(item);
                frame[5] = item.wrapping_sub(0x14);
                let header = rd32(item.wrapping_sub(4));
                callee_thiscall!(
                    CAL_SLOT_SUBMIT, u32, obj,
                    header,
                    frame.as_ptr() as u32,
                    0xFFFF_FFFFu32, 0, 0
                );
                i += 1;
                item = item.wrapping_add(Q1_STRIDE);
                if !(i < rd32(obj.wrapping_add(Q1_COUNT)) as i32) {
                    break;
                }
            }
        }
        wr32(obj.wrapping_add(Q1_COUNT), 0);
        // Queue two: fill, flag and finalise every pending item.
        let mut j = 0i32;
        if (rd32(obj.wrapping_add(Q2_COUNT)) as i32) > 0 {
            let mut item = obj.wrapping_add(Q2_BASE);
            loop {
                let shape = item.wrapping_sub(Q2_STRUCT_BACK);
                callee_thiscall!(CAL_ITEM_FILL, u32, obj, shape, 0);
                // Two voice pointers per item; identical handling.
                for k in 0..2u32 {
                    let slot = if k == 0 { item.wrapping_sub(4) } else { item };
                    let voice = rd32(slot);
                    if voice != 0 {
                        let class = rd8(voice.wrapping_add(VOICE_CLASS));
                        if class == CLASS_FAULT {
                            set_voice_flag(0);
                        } else {
                            let index = rd8(voice.wrapping_add(VOICE_INDEX));
                            let stride = g32(STRIDE_PTR);
                            let table = g32(TABLE_PTR);
                            let entry = rd32(
                                table
                                    .wrapping_add((index as u32).wrapping_mul(TABLE_ENTRY_STRIDE))
                                    .wrapping_add(TABLE_BIAS),
                            );
                            let base =
                                stride.wrapping_mul(class as u32).wrapping_add(entry);
                            set_voice_flag(base);
                        }
                    }
                }
                callee_cdecl!(CAL_ITEM_FINI, u32, shape, 0, 0x80);
                j += 1;
                item = item.wrapping_add(Q2_STRIDE);
                if !(j < rd32(obj.wrapping_add(Q2_COUNT)) as i32) {
                    break;
                }
            }
        }
        wr32(obj.wrapping_add(Q2_COUNT), 0);
        0
    }
});
