// original: 0x009625e0 table_a_init_chain
/// Initialise handle table A, run the object teardown, init table C.
///
/// Sets all `0x818` entries of the table at `0x1208970` to pointer `0` with
/// tag `0xFFFF`, calls the object-table teardown helper (no arguments;
/// intercepted), then initialises all `0x400` 20-byte entries at `0x11FA028`
/// to pointer `0`, tag `0xFFFF`, two zero dwords and a zero word, leaving
/// bytes `+6..+7` and `+18..+19` untouched. Returns the end pointer.
lf_checker_rt::export!(cdecl, rw_009625e0() -> u32 {
    unsafe {
        const TAB_A: u32 = 0x1208970;
        const ENTRIES_A: u32 = 0x818;
        const TAB_C: u32 = 0x11fa028;
        const ENTRIES_C: u32 = 0x400;
        const STRIDE_C: u32 = 0x14;
        let mut i = 0u32;
        while i < ENTRIES_A {
            let e = lf_checker_rt::relocated(TAB_A).wrapping_add(i.wrapping_mul(8));
            (e as *mut u32).write_unaligned(0);
            (e.wrapping_add(4) as *mut u16).write_unaligned(0xffff);
            i += 1;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let mut j = 0u32;
        while j < ENTRIES_C {
            let e = lf_checker_rt::relocated(TAB_C).wrapping_add(j.wrapping_mul(STRIDE_C));
            (e as *mut u32).write_unaligned(0);
            (e.wrapping_add(4) as *mut u16).write_unaligned(0xffff);
            (e.wrapping_add(8) as *mut u32).write_unaligned(0);
            (e.wrapping_add(12) as *mut u32).write_unaligned(0);
            (e.wrapping_add(16) as *mut u16).write_unaligned(0);
            j += 1;
        }
        lf_checker_rt::relocated(0x11fa030).wrapping_add(ENTRIES_C.wrapping_mul(STRIDE_C))
    }
});
