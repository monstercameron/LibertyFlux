// original: 0x00648160 rage::ptxEmitRuleStd::vf16

/// Clear the selector lane's head and tail words (`+0x4` and `+0x14` past
/// `sel * 4`, the selector byte past the global table pointer) and, when
/// `cb` is non-null, notify through its virtual slot `+0x24` with `dst`.
/// Stdcall, two stack arguments, no return value.
lf_checker_rt::export!(stdcall, rw_00648160(dst: u32, cb: u32) -> u32 {
    unsafe {
        const SELECTOR_TABLE_PTR: u32 = 0x01bb_6678;
        const SELECTOR_BYTE: u32 = 0x01;
        const LANE_HEAD: u32 = 0x04;
        const LANE_TAIL: u32 = 0x14;
        const NOTIFY_SLOT: u32 = 0x24;
        let table =
            (lf_checker_rt::relocated(SELECTOR_TABLE_PTR) as *const u32)
                .read_unaligned();
        let sel =
            ((table.wrapping_add(SELECTOR_BYTE)) as *const u8).read() as u32;
        let lane = dst.wrapping_add(sel.wrapping_mul(4));
        (lane.wrapping_add(LANE_HEAD) as *mut u32).write_unaligned(0);
        (lane.wrapping_add(LANE_TAIL) as *mut u32).write_unaligned(0);
        if cb == 0 {
            return 0;
        }
        let vtable = (cb as *const u32).read_unaligned();
        let slot =
            ((vtable.wrapping_add(NOTIFY_SLOT)) as *const u32).read_unaligned();
        let notify: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        notify(cb, dst);
        0
    }
});
