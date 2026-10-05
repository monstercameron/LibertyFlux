// original: 0x009C1520 task_tick_secondary (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, global, relocated};

const TASK_KEY: u32 = 0x14;
const TASK_MODE: u32 = 0x18;
const TASK_DEADLINE: u32 = 0x28;
const TASK_OBJ: u32 = 0x10;
const TASK_OBJ2: u32 = 0x1c;
const HIT_SCORE: u32 = 0xa4;
const HIT_INFO: u32 = 0xec;
const OBJ_KIND: u32 = 0x28;
const KIND_MASK: u32 = 0x3c0;
const KIND_WANT: u32 = 0x0c0;
const OBJ_FLAG_B: u32 = 0x219;
const OBJ_OPTS: u32 = 0x26c;
const OPTS_BIT: u8 = 4;
const OBJ_LINK: u32 = 0xb30;
const LINK_STATE: u32 = 0x1304;
const LINK_WANT: u32 = 2;
const OBJ_NIBBLE: u32 = 0x1e2;
const HUB: u32 = 0x13b6798;
const G_MODE: u32 = 0x11d6fd4;
const G_CLOCK: u32 = 0x11735b4;
const G_RATE: u32 = 0x103ac20;
const C_SCORE_MIN: u32 = 0xfe8628;
const C_RATE_DFLT: u32 = 0xfe88e8;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

/// Secondary per-tick update of one task record.
///
/// `task` points to the record. The record's word at `+0x14` is a key passed
/// to the lookup callee; `+0x18` is a mode, `+0x28` a deadline timestamp,
/// `+0x10` a linked object, `+0x1c` an optional second object.
///
/// Behaviour: resolve the key through callee 1. When the global mode is at
/// least 2, the record mode is 3, the lookup hit, and the hit's `+0xa4` word
/// (converted integer to float) exceeds a constant, and the global clock is
/// past the deadline but within one callee-2 tick of it, notify through
/// callee 3 and re-read the global mode. Then derive a blend flag from the
/// second object (present, kind bits `0x3c0 == 0x0c0` at `+0x28`, nonzero
/// byte at `+0x219`, and, for the rate select, bit 2 at `+0x26c` plus a live
/// `+0xb30` link whose `+0x1304` word is 2). When the mode is at least 2, the
/// record mode is 4 and the low nibble of the linked object's `+0x1e2` byte
/// is 2 or more, return early. Otherwise resolve the key again (a miss here
/// faults on the `[eax+0xec]` read, like the original) and report through
/// callee 4 with the linked object, the saved-entry-ESI slot (low byte
/// replaced by the flag; compared low-byte-only, see contract `call_mask`),
/// the hit's `+0xec` word and the selected rate. The hub object address the
/// original moves into ECX is a relocated immediate, so the rewrite derives
/// it with `relocated()` like every other address.
///
/// The original also stores the selected rate into its own incoming-arg
/// stack slot as scratch. A Rust rewrite cannot address its caller's stack
/// slot, so the contract disables the stack check (recorded narrowing);
/// the rate value itself is still compared exactly as callee 4's argument.
///
/// Original: 0x009C1520 (cdecl, one stack word). Returns the last callee
/// answer, or the masked nibble value on the early path.
lf_checker_rt::export!(cdecl, rw_009C1520(task: u32) -> u32 {
    unsafe {
        let (mode, rate_sel, slot_lo) = rw_009c1520_head(task);
        rw_009c1520_tail(task, mode, rate_sel, slot_lo)
    }
});

/// Head of the function: the lookup, the conditional notify through callee
/// 3, and the two scratch values (`rate_sel`, `slot_lo`). The original holds
/// them in DL and a stack slot respectively.
unsafe fn rw_009c1520_head(task: u32) -> (u32, u8, u8) {
    unsafe {
        let hit: u32 = callee_cdecl!(1, u32, rd32(task + TASK_KEY));
        let mut mode: u32 = global::<u32>(G_MODE).read_unaligned();
        if mode >= 2 && rd32(task + TASK_MODE) == 3 && hit != 0 {
            // int -> float convert, then `comiss`/`jbe`: continue only when
            // strictly greater (NaN takes the skip path on both sides).
            let s = rd32(hit + HIT_SCORE) as i32 as f32;
            let min = f32::from_bits(global::<u32>(C_SCORE_MIN).read_unaligned());
            if s > min {
                let now = global::<u32>(G_CLOCK).read_unaligned();
                if now > rd32(task + TASK_DEADLINE) {
                    let dt: u32 = callee_cdecl!(2, u32,);
                    if now.wrapping_sub(dt) <= rd32(task + TASK_DEADLINE) {
                        let _: u32 =
                            callee_thiscall!(3, u32, relocated(HUB), rd32(task + TASK_OBJ));
                        mode = global::<u32>(G_MODE).read_unaligned();
                    }
                }
            }
        }

        let obj2 = rd32(task + TASK_OBJ2);
        let outer = obj2 != 0
            && rd32(obj2 + OBJ_KIND) & KIND_MASK == KIND_WANT
            && rd8(obj2 + OBJ_FLAG_B) != 0;
        let mut rate_sel: u8 = 0;
        if outer && rd8(obj2 + OBJ_OPTS) & OPTS_BIT != 0 {
            let link = rd32(obj2 + OBJ_LINK);
            if link != 0 && rd32(link + LINK_STATE) == LINK_WANT {
                rate_sel = 1;
            }
        }
        let slot_lo: u8 = if outer { 1 } else { 0 };
        (mode, rate_sel, slot_lo)
    }
}

/// Tail of the function: the mode-4 early return and the final report call.
unsafe fn rw_009c1520_tail(task: u32, mode: u32, rate_sel: u8, slot_lo: u8) -> u32 {
    unsafe {
        if mode >= 2 && rd32(task + TASK_MODE) == 4 {
            let obj = rd32(task + TASK_OBJ);
            // The original loads the object pointer into EAX and replaces
            // only AL, so the early-path return value keeps the pointer's
            // top three bytes.
            let nib = rd8(obj + OBJ_NIBBLE) & 0x0f;
            if nib >= 2 {
                return (obj & 0xffff_ff00) | nib as u32;
            }
        }
        let rate: u32 = if rate_sel != 0 {
            global::<u32>(G_RATE).read_unaligned()
        } else {
            global::<u32>(C_RATE_DFLT).read_unaligned()
        };
        // The original stores the rate into its own incoming-arg stack slot
        // as scratch and reads it back after the lookup call. A Rust rewrite
        // cannot address its caller's stack slot, so the value is carried in
        // a local; the observable call arguments are identical.
        let hit2: u32 = callee_cdecl!(1, u32, rd32(task + TASK_KEY));
        let info = rd32(hit2.wrapping_add(HIT_INFO));
        // ESI slot: the original passes (entry_ESI & ~0xFF) | slot_lo. Entry
        // ESI is not observable from a Rust rewrite, so the rewrite passes
        // the low byte alone and the contract masks this argument to 0xFF.
        let esi_slot = slot_lo as u32;
        callee_thiscall!(
            4,
            u32,
            relocated(HUB),
            rd32(task + TASK_OBJ),
            esi_slot,
            info,
            rate
        )
    }
}
