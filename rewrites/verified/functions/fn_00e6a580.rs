// original: 0x00e6a580 veh_table_init_dispatch_03 (proposed)

/// Calls a method on every entry of a vehicle object table, then notifies the dispatcher.
///
/// `COUNT` entries of `STRIDE` bytes starting at `BASE`; each entry
/// address goes in ECX with no stack words (thiscall/0),
/// /// then the dispatcher (a one-word cdecl callee) with `TARGET`. All
/// results are discarded. Takes no
/// arguments, returns nothing (cdecl/0).
///
/// Original: 0x00e6a580 (cdecl, no arguments, 17 calls).
lf_checker_rt::export!(cdecl, rw_00e6a580() -> () {
    unsafe {
        /// First table entry (file VA).
        const BASE: u32 = 0x016C1FD0;
        /// Number of entries.
        const COUNT: u32 = 0x10;
        /// Entry size in bytes.
        const STRIDE: u32 = 0x1D0;
        /// Entry-method callee id.
        const METHOD: u32 = 1;
        let mut entry = lf_checker_rt::relocated(BASE);
        for _ in 0..COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(METHOD, u32, entry);
            entry = entry.wrapping_add(STRIDE);
        }
        /// Forwarded address (file VA).
        const TARGET: u32 = 0x00E727C0;
        /// Dispatcher callee id.
        const DISPATCH: u32 = 2;
        let _: u32 = lf_checker_rt::callee_cdecl!(DISPATCH, u32, lf_checker_rt::relocated(TARGET));
    }
});
