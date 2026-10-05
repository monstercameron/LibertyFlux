// original: 0x00e69c40 veh_table_loop_03 (proposed)

/// Calls a method on every entry of a vehicle object table.
///
/// `COUNT` entries of `STRIDE` bytes starting at `BASE`; each entry
/// address goes in ECX with no stack words (thiscall/0). All results
/// are discarded. Takes no arguments, returns nothing (cdecl/0).
///
/// Original: 0x00E69C40 (cdecl, no arguments, 64 calls).
lf_checker_rt::export!(cdecl, rw_00e69c40() -> () {
    unsafe {
        /// First table entry (file VA).
        const BASE: u32 = 0x0167CEA0;
        /// Number of entries.
        const COUNT: u32 = 0x40;
        /// Entry size in bytes.
        const STRIDE: u32 = 0x50;
        /// Entry-method callee id.
        const METHOD: u32 = 1;
        let mut entry = lf_checker_rt::relocated(BASE);
        for _ in 0..COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(METHOD, u32, entry);
            entry = entry.wrapping_add(STRIDE);
        }
    }
});
