// original: 0x00A8F8B0 interior_resolve_portal_indices

/// Resolve portal data through two page helpers. The signed 16-bit handle at
/// byte offset `0x2e` selects a pool entry whose page pointer is at `+0x70`.
/// The first helper receives `a`; its result is stored through `out`. The
/// second helper receives `a`, `c`, and `d`, and its result is returned.
/// Thiscall cleans the four stack arguments after the function returns.
lf_checker_rt::export!(thiscall, rw_00a8f8b0(this: u32, a: u32, out: u32, c: u32, d: u32) -> u32 {
    unsafe {
        const HANDLE_OFFSET: u32 = 0x2e;
        const POOL_POINTER_TABLE: u32 = 0x0129_5cd8;
        const PAGE_POINTER_OFFSET: u32 = 0x70;
        const FIRST_HELPER: u32 = 1;
        const SECOND_HELPER: u32 = 2;

        let handle = ((this + HANDLE_OFFSET) as *const i16).read_unaligned();
        let table_slot = lf_checker_rt::relocated(POOL_POINTER_TABLE)
            .wrapping_add((handle as i32 as u32).wrapping_mul(4));
        let pool_entry = (table_slot as *const u32).read_unaligned();
        let page = ((pool_entry + PAGE_POINTER_OFFSET) as *const u32).read_unaligned();
        let value: u32 = lf_checker_rt::callee_thiscall!(FIRST_HELPER, u32, page, a);
        (out as *mut u32).write_unaligned(value);
        lf_checker_rt::callee_thiscall!(SECOND_HELPER, u32, page, a, c, d)
    }
});
