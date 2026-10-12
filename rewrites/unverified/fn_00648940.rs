// original: 0x00648940 rage::ptxEmitRuleStd::vf17

/// When the selector lane's tail word (`+0x14` past `sel * 4`, the
/// selector byte two past the global table pointer) is non-zero and `cb`
/// is non-null, notify through the callback's virtual slot `+0x38` with
/// (`a0`, `dst`, `a3`). Otherwise returns at once. Stdcall, four stack
/// arguments, no return value.
lf_checker_rt::export!(stdcall, rw_00648940(a0: u32, dst: u32, cb: u32, a3: u32) -> u32 {
    unsafe {
        const SELECTOR_TABLE_PTR: u32 = 0x01bb_6678;
        const SELECTOR_BYTE: u32 = 0x02;
        const LANE_TAIL: u32 = 0x14;
        const NOTIFY_SLOT: u32 = 0x38;
        let table =
            (lf_checker_rt::relocated(SELECTOR_TABLE_PTR) as *const u32)
                .read_unaligned();
        let sel =
            ((table.wrapping_add(SELECTOR_BYTE)) as *const u8).read() as u32;
        let tail = (dst
            .wrapping_add(sel.wrapping_mul(4))
            .wrapping_add(LANE_TAIL) as *const u32)
            .read_unaligned();
        if tail == 0 {
            return 0;
        }
        if cb == 0 {
            return 0;
        }
        let vtable = (cb as *const u32).read_unaligned();
        let slot =
            ((vtable.wrapping_add(NOTIFY_SLOT)) as *const u32).read_unaligned();
        let notify: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        notify(cb, a0, dst, a3);
        0
    }
});
