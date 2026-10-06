// original: 0x0069E8A0 dispatch_table_by_flag
/// Call one of two callees over a four-entry global table, by a flag byte.
///
/// When the global byte at FLAG is zero, calls callee 0 as
/// `callee(entry, index, arg)` for the four table entries (index 0..3);
/// otherwise calls callee 1 as `callee(entry, arg)`. Entries are 0x38 bytes
/// at TABLE..TABLE_END (the end comparison is signed, `jl`, though both
/// addresses are always positive). Both callees clean their stack arguments.
/// Returns the last callee's answer. Original: cdecl, one stack word, two
/// direct callees (patched and scripted by the checker).
lf_checker_rt::export!(cdecl, rw_0069e8a0(arg: u32) -> u32 {
    unsafe {
        const FLAG: u32 = 0x018B_7A79;
        const TABLE: u32 = 0x019F_2388;
        const TABLE_END: u32 = 0x019F_2468;
        const STRIDE: u32 = 0x38;
        let flag = lf_checker_rt::global::<u8>(FLAG).read_unaligned();
        let mut last = 0u32;
        if flag == 0 {
            let mut entry = lf_checker_rt::relocated(TABLE);
            let end = lf_checker_rt::relocated(TABLE_END);
            let mut idx = 0u32;
            while (entry as i32) < (end as i32) {
                last = lf_checker_rt::callee_thiscall!(0, u32, entry, idx, arg);
                entry = entry.wrapping_add(STRIDE);
                idx += 1;
            }
        } else {
            let mut entry = lf_checker_rt::relocated(TABLE);
            let end = lf_checker_rt::relocated(TABLE_END);
            while (entry as i32) < (end as i32) {
                last = lf_checker_rt::callee_thiscall!(1, u32, entry, arg);
                entry = entry.wrapping_add(STRIDE);
            }
        }
        last
    }
});