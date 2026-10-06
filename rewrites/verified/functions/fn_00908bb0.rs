// original: 0x00908bb0 input_handle_dispatch (proposed)
/// Dispatch a validated handle by its kind: refresh or latch.
///
/// `kind` selects the path, `handle` is validated through the lookup
/// callee (a negative answer, signed, or a null slot returns the index),
/// and `dst` receives the work. Both live paths first ask the measure
/// callee for a length and clear the halfword at `dst + 0x3c` when it is
/// above `0x1E` (unsigned `jbe`), then run the apply callee on
/// `(index, dst)`. The refresh kind (`0x12`) additionally runs the settle
/// callee on `(index, 0x40)` and returns its answer; the latch kind
/// (`0x11`) instead sets bit 6 of the slot object's word at `+0x20`
/// (falling back to the static default index when the kind byte at `+8`
/// is zero) and returns the object. Any other kind returns `kind - 0x12`.
/// Cdecl with three stack words.
export!(cdecl, rw_00908bb0(kind: u32, handle: u32, dst: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        /// Static default-slot index word (file VA).
        const DEFAULT: u32 = 0x01034494;
        const KIND_REFRESH: u32 = 0x12;
        const KIND_LATCH: u32 = 0x11;
        const LEN_LIMIT: u32 = 0x1E;
        const LEN_OFF: u32 = 0x3C;
        const KIND_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x20;
        const FLAG_BIT: u16 = 0x40;
        const SETTLE_TAG: u32 = 0x40;
        const LOOKUP_ID: u32 = 1;
        const MEASURE_ID: u32 = 2;
        const APPLY_ID: u32 = 3;
        const SETTLE_ID: u32 = 4;
        let idx: u32 = callee_cdecl!(LOOKUP_ID, u32, handle);
        // Signed: the original returns early on the sign flag (js).
        if (idx as i32) < 0 {
            return idx;
        }
        let slot = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if slot == 0 {
            return idx;
        }
        if kind != KIND_LATCH && kind != KIND_REFRESH {
            return kind.wrapping_sub(KIND_REFRESH);
        }
        let len: u32 = callee_cdecl!(MEASURE_ID, u32, dst);
        // Unsigned: the original uses jbe against the limit.
        if len > LEN_LIMIT {
            ((dst.wrapping_add(LEN_OFF)) as *mut u16).write_unaligned(0);
        }
        let _: u32 = callee_cdecl!(APPLY_ID, u32, idx, dst);
        if kind == KIND_REFRESH {
            callee_cdecl!(SETTLE_ID, u32, idx, SETTLE_TAG)
        } else {
            let mut obj = ((relocated(TABLE).wrapping_add(idx.wrapping_mul(4)))
                as *const u32)
                .read_unaligned();
            if ((obj.wrapping_add(KIND_OFF)) as *const u8).read() == 0 {
                let d = (global::<u32>(DEFAULT)).read_unaligned();
                obj = ((relocated(TABLE).wrapping_add(d.wrapping_mul(4))) as *const u32)
                    .read_unaligned();
            }
            let f = ((obj.wrapping_add(FLAG_OFF)) as *const u16).read_unaligned();
            ((obj.wrapping_add(FLAG_OFF)) as *mut u16).write_unaligned(f | FLAG_BIT);
            obj
        }
    }
});
