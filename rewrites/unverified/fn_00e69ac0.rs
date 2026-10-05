// original: 0x00e69ac0 veh_table_loop_02 (proposed)

/// Calls a method on every entry of a vehicle object table.
///
/// `COUNT` entries of `STRIDE` bytes starting at `BASE`; each entry
/// address goes in ECX with no stack words (thiscall/0). All results
/// are discarded. Takes no arguments, returns nothing (cdecl/0).
///
/// Original: 0x00E69AC0 (cdecl, no arguments, 20 calls).
lf_checker_rt::export!(cdecl, rw_00e69ac0() -> () {
    unsafe {
        /// First table entry (file VA).
        const BASE: u32 = 0x01670550;
        /// Number of entries.
        const COUNT: u32 = 0x14;
        /// Entry size in bytes.
        const STRIDE: u32 = 0x60;
        /// Entry-method callee id.
        const METHOD: u32 = 1;
        let mut entry = lf_checker_rt::relocated(BASE);
        for _ in 0..COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(METHOD, u32, entry);
            entry = entry.wrapping_add(STRIDE);
        }
    }
});
