// original: 0x008e0880 pool_indexed_store (proposed)

/// Store a scaled table word plus the successor index through `out`.
///
/// Resolves slot `index` through the pool context at `CTX` (entry base at
/// `+0x00`, flag bytes at `+0x04`, stride at `+0x0c`), faulting through null
/// when its `0x80` flag bit is set, and reads the successor index at entry
/// `+0x0c`, returning 0 when it is -1. Otherwise runs the refresh (callee 1)
/// on the pool context, then stores `table[scale * 100 + answer + 0x58] +
/// successor` through `out` (where `scale` is the global at `SCALE` and
/// `answer` is the refresh call's return value, used as the row base) and
/// returns 1. Cdecl, two stack arguments.
lf_checker_rt::export!(cdecl, rw_008e0880(index: u32, out: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x0117_64c0;
        const SCALE: u32 = 0x0103_2f58;
        const FLAG_BASE_OFF: u32 = 0x04;
        const STRIDE_OFF: u32 = 0x0c;
        const NEXT_OFF: u32 = 0x0c;
        const ROW_OFF: u32 = 0x58;
        const ROW_STRIDE: u32 = 100;
        const DEAD_FLAG: u8 = 0x80;
        const NO_NEXT: u32 = 0xffff_ffff;
        const CALLEE_REFRESH: u32 = 1;
        let ctx = lf_checker_rt::global::<u32>(CTX).read_unaligned();
        let flag_base = ((ctx + FLAG_BASE_OFF) as *const u32).read_unaligned();
        let stride = ((ctx + STRIDE_OFF) as *const u32).read_unaligned();
        let base = (ctx as *const u32).read_unaligned();
        let entry = if (flag_base.wrapping_add(index) as *const u8).read() & DEAD_FLAG != 0 {
            0
        } else {
            base.wrapping_add(stride.wrapping_mul(index))
        };
        let next = ((entry + NEXT_OFF) as *const u32).read_unaligned();
        if next == NO_NEXT {
            return 0;
        }
        let scale = lf_checker_rt::global::<u32>(SCALE).read_unaligned();
        let answer = lf_checker_rt::callee_thiscall!(CALLEE_REFRESH, u32, ctx);
        let cell_addr = scale
            .wrapping_mul(ROW_STRIDE)
            .wrapping_add(answer)
            .wrapping_add(ROW_OFF);
        let cell = (cell_addr as *const u32).read_unaligned();
        (out as *mut u32).write_unaligned(cell.wrapping_add(next));
        1
    }
});
