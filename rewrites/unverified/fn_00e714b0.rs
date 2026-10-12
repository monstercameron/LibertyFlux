// original: 0x00E714B0 initialize_array_256_entries
/// Initialize 256 entries in reverse order with the four-argument registration helper.
lf_checker_rt::export!(cdecl, rw_fn_00E714B0() -> () {
    unsafe {
        const FIRST_ENTRY: u32 = 0x011A9D18;
        const ENTRY_STRIDE: u32 = 0x14;
        const ENTRY_COUNT: u32 = 256;
        const ENTRY_KIND: u32 = 4;
        const ENTRY_FLAGS: u32 = 5;
        const ENTRY_DESCRIPTOR: u32 = 0x0093E2C0;

        let mut entry = lf_checker_rt::relocated(FIRST_ENTRY);
        for _ in 0..ENTRY_COUNT {
            entry = entry.wrapping_sub(ENTRY_STRIDE);
            let _ = lf_checker_rt::callee_stdcall!(1, u32, entry, ENTRY_KIND, ENTRY_FLAGS, lf_checker_rt::relocated(ENTRY_DESCRIPTOR));
        }
    }
});
