// original: 0x008C97C0 stream_bind_slot (proposed)

/// Binds streaming slot `slot` and reports its channel: when `flag` is
/// non-zero the slot is replaced by the global `CURRENT` slot unless it
/// already equals it. Then calls the bind callee (callee 0, cdecl with the
/// table address `TABLE`, the slot's row address `ROWS + slot * ROW_STRIDE`
/// and the row length 0x3C), stores the slot's channel word from
/// `CHANNELS[slot]` into `*out`, and returns `TABLE`.
///
/// Three stack arguments (cdecl); the table address is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008C97C0(slot: u32, flag: u32, out: u32) -> u32 {
    unsafe {
        /// Global currently bound slot.
        const CURRENT: u32 = 0x1031BB8;
        /// Base of the slot row storage.
        const ROWS: u32 = 0x116D298;
        /// Stride between slot rows.
        const ROW_STRIDE: u32 = 0x134;
        /// Row length passed to the bind callee.
        const ROW_LEN: u32 = 0x3C;
        /// Table address passed to the callee and returned.
        const TABLE: u32 = 0x11734E8;
        /// Base of the per-slot channel words.
        const CHANNELS: u32 = 0x1172D50;
        /// Bind callee id.
        const BIND: u32 = 0;
        let mut s = slot;
        if flag & 0xFF != 0 {
            let cur = (lf_checker_rt::relocated(CURRENT) as *const u32).read();
            if s != cur {
                s = cur;
            }
        }
        let table = lf_checker_rt::relocated(TABLE);
        let row = lf_checker_rt::relocated(ROWS).wrapping_add(s.wrapping_mul(ROW_STRIDE));
        let _b: u32 = lf_checker_rt::callee_cdecl!(BIND, u32, table, row, ROW_LEN);
        let ch = (lf_checker_rt::relocated(CHANNELS).wrapping_add(s.wrapping_mul(4)) as *const u32)
            .read();
        (out as *mut u32).write_unaligned(ch);
        table
    }
});
