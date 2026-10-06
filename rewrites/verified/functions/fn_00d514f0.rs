// original: 0x00d514f0 CRenderPhaseInteriorReflection::vf7
//
// Interior-reflection render phase packet pass (thiscall, no stack words).
//
// Gated by a global activity byte: when clear it returns entry EAX (0 under
// the contract) at once. Otherwise it saves `this` in a global slot, emits
// two allocated packets through makers into the command sink, optionally
// emits a pointer-maker packet gated by the object's mode byte at +0x1c,
// dispatches virtual slots +0x24 and +0x20 through `this` and slot +0x2c
// through the sub-object at +0x890 (that answer bounds a packet loop), then
// per loop iteration emits maker packets, feeds a frame of loop state plus
// a float level (the override global when it compares greater than +0.0,
// else the default; NaN and signed zeros keep the default) to the
// descriptor consumer, and emits a descriptor packet. Past the loop it
// cookie-tags one packet with the image's own function table and dispatches
// that table's slot twice through it, then emits the trailing packet whose
// stamp is the return value. Each stamp folds two slot answers with the
// signed remainder-mod-16 idiom and a truncating divide by 16 shifted into
// a mask field xored over the packet word. A null allocation, or a skipped
// maker, faults at the unconditional slot use, identically on both sides.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Callee ids (see contract).
const C_ALLOC: u32 = 1; // 0x8dc3a0 cdecl/2: packet allocator
const C_MAKE1: u32 = 2; // 0x8dbec0 thiscall/1: answer initializer
const C_SINK: u32 = 3; // 0x499e30 cdecl/1: command sink
const C_MAKE4: u32 = 4; // 0x8dbe70 thiscall/4: packet maker
const C_MAKEPTR: u32 = 5; // 0x8dc0d0 thiscall/1: pointer maker
const C_MAKEFR: u32 = 6; // 0x8dc2c0 thiscall/2: descriptor maker (frame arg)
const C_MAKE0: u32 = 7; // 0x8bbd0 thiscall/0: trailing maker
const C_DESC: u32 = 8; // 0xd50d50 cdecl/5: descriptor consumer (frame args)
// Ids 9-13 are slot calls through planted tables (no direct patch).

// Globals (file VAs).
const G_GATE: u32 = 0x01720FA8; // main gate byte
const G_SAVE: u32 = 0x012FB1B8; // this-pointer save slot, cleared on exit
const G_FA: u32 = 0x0103F9A4; // default level (float bits)
const G_FB: u32 = 0x010552C8; // override level (float bits)
const G_COOKIE: u32 = 0x010327A0; // packet cookie counter

// Relocated addresses (all carry HIGHLOW relocs).
const TAG_TMP: u32 = 0x00E7E048; // temporary tag during cookie-tagging
const TAG_IMG: u32 = 0x00E8668C; // image function table installed into the packet
const DESC_ARG: u32 = 0x00AD2800; // descriptor id handed to the consumer

const MASK: u32 = 0x01FF_C000; // stamped mix mask
const F_ONE: u32 = 0x3F80_0000; // 1.0f bits
const F_N1B: u32 = 0xFF00_0000; // descriptor constant word

#[inline(always)]
unsafe fn rd32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn wr32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn rg32(va: u32) -> u32 {
    unsafe { global::<u32>(va).read() }
}

#[inline(always)]
unsafe fn wg32(va: u32, v: u32) {
    unsafe { global::<u32>(va).write(v) }
}

#[inline(always)]
unsafe fn rb8(va: u32) -> u8 {
    unsafe { global::<u8>(va).read() }
}

#[inline(always)]
unsafe fn robj(obj: u32, off: u32) -> u8 {
    unsafe { ((obj.wrapping_add(off)) as *const u8).read() }
}

/// Call virtual slot `slot` of `obj` exactly like the original: load the
/// table pointer from the object, load the slot, call with obj in ECX.
/// Both sides land on the same planted recorder stub.
#[inline(always)]
unsafe fn call_slot0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let vtbl = (obj as *const u32).read_unaligned();
        let target = ((vtbl.wrapping_add(slot)) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        f(obj)
    }
}

/// C-style signed remainder mod 16 (matches the and-with-fixup idiom).
#[inline(always)]
fn smod16(v: u32) -> i32 {
    (v as i32) % 16
}

/// Fold one slot-answer pair into the stamped mix: the second answer plus
/// the complement of the first answer's remainder, divided truncating by
/// 16, shifted into the stamp field.
#[inline(always)]
fn mix_pair(r1: u32, r2: u32) -> u32 {
    let t = (16 - smod16(r1)) % 16;
    ((r2.wrapping_add(t as u32) as i32) / 16).wrapping_shl(14) as u32
}

/// Stamp the mix of two slot answers into the packet's word at +4, xoring
/// the masked mix over the old value. Returns the masked mix (the value
/// the original holds in EAX after the stamp).
#[inline(always)]
unsafe fn stamp(obj: u32, r1: u32, r2: u32) -> u32 {
    unsafe {
        let old = rd32(obj.wrapping_add(4));
        let x = (mix_pair(r1, r2) ^ old) & MASK;
        wr32(obj.wrapping_add(4), old ^ x);
        x
    }
}

/// Cookie-tag the image-table packet: temporary tag, counter folded into
/// the residue word, counter bumped, then the image table installed.
/// Mirrors the original's exact store order.
#[inline(always)]
unsafe fn tag_image_packet(p: u32) {
    unsafe {
        let residue = rd32(p.wrapping_add(4));
        wr32(p, relocated(TAG_TMP));
        let cookie = (residue ^ rg32(G_COOKIE)) & 0x3FFF;
        wr32(p.wrapping_add(4), residue ^ cookie);
        wg32(G_COOKIE, rg32(G_COOKIE).wrapping_add(1));
        wr32(p, relocated(TAG_IMG));
    }
}

export!(thiscall, rw_00d514f0(this_ptr: u32) -> u32 {
    unsafe {
        if rb8(G_GATE) == 0 {
            return 0; // entry EAX, fixed to 0 by the contract
        }
        wg32(G_SAVE, this_ptr);
        let sub_addr = this_ptr.wrapping_add(0x890);
        // Block 1: alloc -> slot +0x24 answer through the initializer.
        let p = callee_cdecl!(C_ALLOC, u32, 0x10, 1);
        let mut ans = if p != 0 {
            let v9 = call_slot0(this_ptr, 0x24);
            callee_thiscall!(C_MAKE1, u32, p, v9)
        } else {
            0
        };
        ans = callee_cdecl!(C_SINK, u32, ans);
        // Block 2: alloc -> 4-word maker over the sub-object words.
        let q = callee_cdecl!(C_ALLOC, u32, 0x18, 0);
        ans = if q != 0 {
            callee_thiscall!(C_MAKE4, u32, q, 0, rd32(sub_addr),
                rd32(this_ptr.wrapping_add(0x898)), 0)
        } else {
            0
        };
        ans = callee_cdecl!(C_SINK, u32, ans);
        // Block 3 (mode-gated): alloc -> pointer maker over this+0xB0.
        if robj(this_ptr, 0x1c) != 0 {
            let r = callee_cdecl!(C_ALLOC, u32, 0x410, 0);
            ans = if r != 0 {
                callee_thiscall!(C_MAKEPTR, u32, r, this_ptr.wrapping_add(0xB0))
            } else {
                0
            };
            ans = callee_cdecl!(C_SINK, u32, ans);
        }
        // Slot +0x20, then the descriptor-word mirror (flag word 1).
        let _v8 = call_slot0(this_ptr, 0x20);
        let mirror = [F_ONE, F_ONE, F_N1B, 0, 0, 0, 1, 0];
        let s = callee_cdecl!(C_ALLOC, u32, 0x2c, 0);
        ans = if s != 0 {
            callee_thiscall!(C_MAKEFR, u32, s, 0, mirror.as_ptr() as u32)
        } else {
            0
        };
        ans = callee_cdecl!(C_SINK, u32, ans);
        // Loop bound from slot +0x2c of the sub-object.
        let sub = rd32(sub_addr);
        let bound = call_slot0(sub, 0x2c);
        let mut fr = [1u32, 0, 2, bound];
        let mut ctr: u32 = 1;
        if (bound as i32) > 1 {
            loop {
                // Block (a): alloc -> 4-word maker over the counter.
                let pa = callee_cdecl!(C_ALLOC, u32, 0x18, 0);
                let oba = if pa != 0 {
                    callee_thiscall!(C_MAKE4, u32, pa, 0, rd32(sub_addr), 0, ctr)
                } else {
                    0
                };
                let r1 = call_slot0(oba, 8);
                let r2 = call_slot0(oba, 8);
                stamp(oba, r1, r2);
                // Block (b) (mode-gated): alloc -> pointer maker.
                if robj(this_ptr, 0x1c) != 0 {
                    let pb = callee_cdecl!(C_ALLOC, u32, 0x410, 0);
                    let obb = if pb != 0 {
                        callee_thiscall!(C_MAKEPTR, u32, pb, this_ptr.wrapping_add(0xB0))
                    } else {
                        0
                    };
                    let r1 = call_slot0(obb, 8);
                    let r2 = call_slot0(obb, 8);
                    stamp(obb, r1, r2);
                }
                // Float level: override when positive, else default.
                let mut level = rg32(G_FA);
                let over = rg32(G_FB);
                if f32::from_bits(over) > 0.0 {
                    level = over;
                }
                fr[0] = ctr;
                fr[1] = level;
                fr[2] = 2;
                fr[3] = bound;
                callee_cdecl!(
                    C_DESC, u32, relocated(DESC_ARG), sub_addr,
                    fr.as_ptr().wrapping_add(2) as u32,
                    fr.as_ptr().wrapping_add(1) as u32,
                    fr.as_ptr() as u32
                );
                // Block (c): alloc -> descriptor maker over the mirror.
                let pc = callee_cdecl!(C_ALLOC, u32, 0x2c, 0);
                let obc = if pc != 0 {
                    callee_thiscall!(C_MAKEFR, u32, pc, 0, mirror.as_ptr() as u32)
                } else {
                    0
                };
                let r1 = call_slot0(obc, 8);
                let r2 = call_slot0(obc, 8);
                stamp(obc, r1, r2);
                ctr = ctr.wrapping_add(1);
                if (ctr as i32) >= (bound as i32) {
                    break;
                }
            }
        }
        // Block (e) (mode-gated): image-table packet, slot twice.
        if robj(this_ptr, 0x1c) != 0 {
            let pe = callee_cdecl!(C_ALLOC, u32, 8, 0);
            let mut oej = 0;
            if pe != 0 {
                tag_image_packet(pe);
                oej = pe;
            }
            let r1 = call_slot0(oej, 8);
            let r2 = call_slot0(oej, 8);
            stamp(oej, r1, r2);
        }
        // Block (f): trailing packet; its stamp is the answer.
        let pf = callee_cdecl!(C_ALLOC, u32, 8, 0);
        let obf = if pf != 0 { callee_thiscall!(C_MAKE0, u32, pf) } else { 0 };
        let r1 = call_slot0(obf, 8);
        let r2 = call_slot0(obf, 8);
        let ret = stamp(obf, r1, r2);
        wg32(G_SAVE, 0);
        ret
    }
});
