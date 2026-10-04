// original: 0x00ae25a0 input_slot_dispatch (proposed)

/// Dispatch one input slot to the sample sink, chosen by slot flags.
///
/// Arguments (cdecl, four stack words): `obj` points to the slot object,
/// `sample` is an opaque 32-bit sample word that is only copied, never
/// computed on, `mask` selects which sink bits are live, and `aux` points to
/// a helper object whose table slot at `+0x24` is called once the slot is
/// known to be active (thiscall, no stack arguments, result ignored).
///
/// When the global init flag is clear, a flag maze on `obj+0x28` bit
/// `0x1000000`, `obj+0x24` bits `0x800000`/`0x8000000` and the mask global
/// may still reach the registration check: a slot whose low flag nibble has
/// bit `0x10` set but not `0x20` is registered through the allocator callee
/// (thiscall on a fixed object, one pushed word `0x10`, result receives the
/// slot pointer) and gains bit `0x20`. A slot with a zero word at `+0x34`
/// ends the call here.
///
/// Otherwise the byte at `+0x63` is the dispatch key (a copy is kept; the
/// original spills it over the incoming argument slot). When the mode global
/// is nonzero, shares a bit with `mask`, and the word at `+0x5c` lacks bit
/// `0x2000`, the key is bumped by `0x10`, saturating at `0xff` once it has
/// reached `0xef`. The helper call runs, then a row pointer is fetched from
/// the global row table indexed by the signed halfword at `+0x2e`, and the
/// saved (pre-bump) key decides:
///
/// - key below `0xef`: the sink callee runs once with (`live`, `obj`,
///   `sample`, code), where `live` is the live-bits global masked down and
///   the code is 7 when the row word at `+0x40` has bit `0x100`, else 4;
/// - key at or above `0xef`: the slot byte is pinned to `0xff` and the sink
///   runs with (`mask`, `obj`, `sample`, code), where the code is 7 when
///   the row bit is set, else 1 for a nonzero byte at `+0x61`, else 0.
///   Four optional follow-ups then run in order, each gated by a bit in
///   `obj+0x24`: `0x1000` with a live sink either routes through the
///   three-argument sink when the masked `obj+0x28` bits equal `0x80`, or,
///   when the row bit is clear, sinks code 6 (a set row bit is rewritten
///   unchanged); `0x2000` with a nonzero latched word sinks code 3;
///   `0x4000` with a nonzero second latch sinks code 2 unless the row bit
///   is set (which is rewritten unchanged); `0x8000` with a live sink
///   sinks code 5.
///
/// The sink takes (target, slot, sample, code); the router takes (slot,
/// sample, target). Every pushed register that the original overwrites with
/// the sample word before the call is dead stack allocation, so the rewrite
/// passes the sample directly. Nothing meaningful is returned.
///
/// Original: 0x00ae25a0 (cdecl, four stack words, no meaningful return).
lf_checker_rt::export!(cdecl, rw_00ae25a0(obj: u32, sample: u32, mask: u32, aux: u32) -> u32 {
    unsafe {
        const OBJ_FLAGS: u32 = 0x24;
        const OBJ_STATE: u32 = 0x28;
        const OBJ_INDEX: u32 = 0x2e;
        const OBJ_GATE: u32 = 0x34;
        const OBJ_SUB: u32 = 0x5c;
        const OBJ_KIND: u32 = 0x61;
        const OBJ_KEY: u32 = 0x63;
        const ROW_BIT_OFF: u32 = 0x40;
        const ROW_BIT: u32 = 0x100;
        const VT_SLOT: u32 = 0x24;
        const KEY_TOP: u32 = 0xef;
        const G_INIT: u32 = 0x015b2b91;
        const G_MASK: u32 = 0x0159af24;
        const G_MODE: u32 = 0x0159af28;
        const G_LATCH0: u32 = 0x0159b754;
        const G_LATCH1: u32 = 0x0159b758;
        const G_LIVE: u32 = 0x0159b750;
        const G_ROWS: u32 = 0x01295cd8;
        const REG_OBJ: u32 = 0x01615470;
        const CALLEE_REG: u32 = 1;
        const CALLEE_HELPER: u32 = 2;
        const CALLEE_SINK: u32 = 3;
        const CALLEE_ROUTE: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }

        let mut last: u32 = 0;
        if rd8(lf_checker_rt::relocated(G_INIT)) == 0 {
            let state = rd32(obj.wrapping_add(OBJ_STATE));
            let mut check = false;
            if state & 0x1000000 != 0 {
                check = true;
            } else {
                let flags = rd32(obj.wrapping_add(OBJ_FLAGS));
                if flags & 0x800000 == 0 {
                    check = true;
                } else if flags & 0x8000000 == 0 {
                    check = true;
                } else if rd32(lf_checker_rt::relocated(G_MASK)) & mask != 0 {
                    check = true;
                }
            }
            if check && state & 0x10 != 0 && state & 0x20 == 0 {
                let slot: u32 = lf_checker_rt::callee_thiscall!(
                    CALLEE_REG,
                    u32,
                    lf_checker_rt::relocated(REG_OBJ),
                    0x10u32
                );
                wr32(slot, obj);
                wr32(
                    obj.wrapping_add(OBJ_STATE),
                    rd32(obj.wrapping_add(OBJ_STATE)) | 0x20,
                );
                last = slot;
            }
        }
        if rd32(obj.wrapping_add(OBJ_GATE)) == 0 {
            return last;
        }
        let key = rd8(obj.wrapping_add(OBJ_KEY)) as u32;
        let mode = rd32(lf_checker_rt::relocated(G_MODE));
        if mode != 0 && mode & mask != 0 && rd16(obj.wrapping_add(OBJ_SUB)) & 0x2000 == 0 {
            if key >= KEY_TOP {
                wr8(obj.wrapping_add(OBJ_KEY), 0xff);
            } else {
                wr8(obj.wrapping_add(OBJ_KEY), (key + 0x10) as u8);
            }
        }
        let latch0 = rd32(lf_checker_rt::relocated(G_LATCH0));
        let latch1 = rd32(lf_checker_rt::relocated(G_LATCH1));
        let table = rd32(aux);
        let helper: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(table.wrapping_add(VT_SLOT)) as usize);
        last = helper(aux);
        let index = rd16(obj.wrapping_add(OBJ_INDEX)) as i16 as i32 as u32;
        let row = rd32(
            lf_checker_rt::relocated(G_ROWS).wrapping_add(index.wrapping_mul(4)),
        );
        let row_word = rd32(row.wrapping_add(ROW_BIT_OFF));
        let live = rd32(lf_checker_rt::relocated(G_LIVE));
        if key < KEY_TOP {
            let code = if row_word & ROW_BIT != 0 { 7u32 } else { 4u32 };
            last = lf_checker_rt::callee_cdecl!(CALLEE_SINK, u32, live & mask, obj, sample, code);
            return last;
        }
        wr8(obj.wrapping_add(OBJ_KEY), 0xff);
        let code = if row_word & ROW_BIT != 0 {
            7u32
        } else if rd8(obj.wrapping_add(OBJ_KIND)) != 0 {
            1u32
        } else {
            0u32
        };
        last = lf_checker_rt::callee_cdecl!(CALLEE_SINK, u32, mask, obj, sample, code);
        let flags = rd32(obj.wrapping_add(OBJ_FLAGS));
        if flags & 0x1000 != 0 && live != 0 {
            if rd32(obj.wrapping_add(OBJ_STATE)) & 0x3c0 == 0x80 {
                last = lf_checker_rt::callee_cdecl!(CALLEE_ROUTE, u32, obj, sample, live & mask);
            } else if row_word & ROW_BIT == 0 {
                last = lf_checker_rt::callee_cdecl!(
                    CALLEE_SINK,
                    u32,
                    live & mask,
                    obj,
                    sample,
                    6u32
                );
            } else {
                wr32(row.wrapping_add(ROW_BIT_OFF), row_word | ROW_BIT);
            }
        }
        if flags & 0x2000 != 0 && latch1 != 0 {
            last = lf_checker_rt::callee_cdecl!(
                CALLEE_SINK,
                u32,
                latch1 & mask,
                obj,
                sample,
                3u32
            );
        }
        if flags & 0x4000 != 0 && latch0 != 0 {
            // Re-read: the earlier sink calls cannot change the row, but the
            // original re-reads the word here, so the rewrite does too.
            let w2 = rd32(row.wrapping_add(ROW_BIT_OFF));
            if w2 & ROW_BIT == 0 {
                last = lf_checker_rt::callee_cdecl!(
                    CALLEE_SINK,
                    u32,
                    latch0 & mask,
                    obj,
                    sample,
                    2u32
                );
            } else {
                wr32(row.wrapping_add(ROW_BIT_OFF), w2 | ROW_BIT);
            }
        }
        if flags & 0x8000 != 0 && live != 0 {
            last = lf_checker_rt::callee_cdecl!(
                CALLEE_SINK,
                u32,
                live & mask,
                obj,
                sample,
                5u32
            );
        }
        last
    }
});
