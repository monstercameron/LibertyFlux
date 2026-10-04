// original: 0x00d29430 CTargetting::vf1 (symbols)

/// Acquire a target into its slot, refreshing a found record.
///
/// Looks up `arg0` with the slot-record finder (intercepted); a miss runs
/// the miss handler (intercepted) instead. On a hit, runs the touch helper
/// (intercepted) on the record and stops when `arg1` is zero. Otherwise sets
/// the record's float at `+0x24` to 5.0, copies the source point's four words
/// (past its link at `arg0 + 0x20` plus `0x30`, or `+0x10` past `arg0` itself
/// when null) into the record, and runs the finish helper (intercepted) on
/// it. Returns the last helper's answer.
///
/// Original: 0x00D29430 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d29430(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const FLOAT_OFF: u32 = 0x24;
        const SET_BITS: u32 = 0x40a0_0000; // 5.0f
        const LINK_OFF: u32 = 0x20;
        const LINKED_PT_OFF: u32 = 0x30;
        const DIRECT_PT_OFF: u32 = 0x10;
        let rec: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, arg0);
        if rec == 0 {
            return lf_checker_rt::callee_thiscall!(4, u32, this, arg0);
        }
        let touched: u32 = lf_checker_rt::callee_thiscall!(2, u32, rec);
        if (arg1 as u8) == 0 {
            return touched;
        }
        unsafe { ((rec + FLOAT_OFF) as *mut u32).write_unaligned(SET_BITS) };
        let link = unsafe { ((arg0 + LINK_OFF) as *const u32).read_unaligned() };
        let src = if link != 0 { link + LINKED_PT_OFF } else { arg0 + DIRECT_PT_OFF };
        for off in [0u32, 4, 8, 12] {
            let v = unsafe { ((src + off) as *const u32).read_unaligned() };
            unsafe { ((rec + off) as *mut u32).write_unaligned(v) };
        }
        lf_checker_rt::callee_thiscall!(3, u32, rec)
    }
});
