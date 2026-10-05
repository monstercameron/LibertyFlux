// original: 0x008a9580 audio_slot_voice_update
//
// Update every live voice slot: walk 800 slots, follow each live slot's
// node chain, flag the resolved entries through three intercepted calls
// and two stage calls, then unlink the chain.
//
// `obj` (ECX) is the voice bank. The outer loop tests one bitmask bit per
// slot (`[obj+0x28a0][i >> 5] & rol(1, i)`); a clear bit or a 0xffff head
// skips the slot. Each chain node at `obj + bx * 4` holds the next link
// (low word) and two tag bytes; the entry pointer is
// `sil * G1 + table[cl]` and the flag pointer `al * G3 + table2[cl2]`
// (both tables hang off the `UP_TABLE_BASE` global). Flag bytes are
// updated with read-modify-write ops, then callees 1-3 run with the entry
// in ECX (callee 1's answer, when nonzero, is zeroed through), then the
// stage pair runs with a frame scratch pointer. Returns the bitmask base
// when the last slot is clear, else 0xffff. Thiscall with no stack args.

use lf_checker_rt::{callee_thiscall, export, global};

/// Global scale multiplied by the chain node's third byte.
const UP_SCALE_NODE: u32 = 0x115d964;
/// Global base of the node lookup tables.
const UP_TABLE_BASE: u32 = 0x115d988;
/// Global scale multiplied by the entry's tag byte.
const UP_SCALE_TAG: u32 = 0x115d968;
/// Bytes per lookup-table row selected by a tag byte.
const UP_ROW_STRIDE: u32 = 0x6f40;
/// Table offsets added to the scaled tag and the table base.
const UP_TAB_NODE_OFF: u32 = 0x6f10;
const UP_TAB_TAG_OFF: u32 = 0x6f14;
/// Object fields: slot-head array, liveness bitmask, spill slot, stage input.
const UP_HEADS_OFF: u32 = 0xfa4;
const UP_MASK_OFF: u32 = 0x28a0;
const UP_SPILL_OFF: u32 = 0x320c;
const UP_STAGE_OFF: u32 = 0x3210;
/// Slots scanned by the outer loop.
const UP_SLOTS: u32 = 0x320;
/// Chain terminator.
const UP_END: u32 = 0xffff;
/// Callee ids (see contract).
const UP_TOUCH: u32 = 1;
const UP_SYNC: u32 = 2;
const UP_RELEASE: u32 = 3;
const UP_BEGIN: u32 = 4;
const UP_COMMIT: u32 = 5;

#[inline(always)]
unsafe fn ru32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn wu32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}
#[inline(always)]
unsafe fn rb(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}
#[inline(always)]
unsafe fn wb(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}
#[inline(always)]
unsafe fn ru16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read_unaligned() }
}
#[inline(always)]
unsafe fn wu16(addr: u32, v: u16) {
    unsafe { (addr as *mut u16).write_unaligned(v) }
}

export!(thiscall, rw_008a9580(obj: u32) -> u32 {
    unsafe {
        let g1 = global::<u32>(UP_SCALE_NODE).read();
        let g2 = global::<u32>(UP_TABLE_BASE).read();
        let g3 = global::<u32>(UP_SCALE_TAG).read();
        let mut scratch = [0u32; 2];
        let mut exit = 0u32;
        let mut i = 0u32;
        let mut bit = 1u32;
        let mut head_ptr = obj.wrapping_add(UP_HEADS_OFF);
        while i < UP_SLOTS {
            let mask = ru32(obj.wrapping_add(UP_MASK_OFF));
            let word = ru32(mask.wrapping_add((i >> 5).wrapping_mul(4)));
            if word & bit == 0 {
                exit = mask;
            } else {
                let head = ru16(head_ptr) as u32;
                if head == UP_END {
                    wu16(head_ptr, UP_END as u16);
                    exit = UP_END;
                } else {
                    let mut first = UP_END;
                    let mut bx = head;
                    loop {
                        let node = obj.wrapping_add(bx.wrapping_mul(4));
                        let prev = bx;
                        bx = ru16(node) as u32;
                        let cl = rb(node.wrapping_add(2)) as u32;
                        let sil = rb(node.wrapping_add(3)) as u32;
                        let entry = sil
                            .wrapping_mul(g1)
                            .wrapping_add(ru32(
                                g2.wrapping_add(cl.wrapping_mul(UP_ROW_STRIDE))
                                    .wrapping_add(UP_TAB_NODE_OFF),
                            ));
                        let tag = rb(entry.wrapping_add(4));
                        wb(entry.wrapping_add(0x38), rb(entry.wrapping_add(0x38)) | 4);
                        // The flag pointer is recomputed before each of the
                        // three updates, exactly like the original; the
                        // updates run clear, set, call, set, call, call.
                        let mut flag_at = [0u32; 3];
                        for pass in 0..3 {
                            let flags = if tag == 0xff {
                                0
                            } else {
                                let cl2 = rb(entry.wrapping_add(0x40)) as u32;
                                (tag as u32).wrapping_mul(g3).wrapping_add(ru32(
                                    g2.wrapping_add(cl2.wrapping_mul(UP_ROW_STRIDE))
                                        .wrapping_add(UP_TAB_TAG_OFF),
                                ))
                            };
                            flag_at[pass] = flags.wrapping_add(0xe8);
                        }
                        // A 0xff tag faults on address 0xe8 in the first
                        // update, on both sides alike.
                        wb(flag_at[0], rb(flag_at[0]) & !2);
                        wb(flag_at[1], rb(flag_at[1]) | 8);
                        let touched = callee_thiscall!(UP_TOUCH, u32, entry);
                        if touched != 0 {
                            wu32(touched, 0);
                        }
                        wb(flag_at[2], rb(flag_at[2]) | 0x10);
                        let _ = callee_thiscall!(UP_SYNC, u32, entry);
                        let _ = callee_thiscall!(UP_RELEASE, u32, entry, 0u32);
                        if first == UP_END {
                            wu16(head_ptr, bx as u16);
                        }
                        let _ = callee_thiscall!(
                            UP_BEGIN,
                            u32,
                            scratch.as_mut_ptr() as u32,
                            obj.wrapping_add(UP_STAGE_OFF)
                        );
                        if prev != UP_END {
                            wu16(node, ru16(obj.wrapping_add(UP_SPILL_OFF)));
                            wu32(obj.wrapping_add(UP_SPILL_OFF), prev);
                        }
                        let _ =
                            callee_thiscall!(UP_COMMIT, u32, scratch.as_mut_ptr() as u32);
                        first = prev;
                        if bx == UP_END {
                            break;
                        }
                    }
                    wu16(head_ptr, UP_END as u16);
                    exit = UP_END;
                }
            }
            bit = bit.rotate_left(1);
            i += 1;
            head_ptr = head_ptr.wrapping_add(8);
        }
        exit
    }
});
