// original: 0x00e6a560 veh_table_init_02 (proposed)

/// Calls a method on every entry of a vehicle object table.
///
/// `COUNT` entries of `STRIDE` bytes starting at `BASE`; each entry
/// address goes in ECX with no stack words (thiscall/0). Takes no
/// arguments, returns nothing (cdecl/0).
///
/// Original: 0x00e6a560 (cdecl, no arguments, 768 calls).
lf_checker_rt::export!(cdecl, rw_00e6a560() -> () {
    unsafe {
        /// First table entry (file VA).
        const BASE: u32 = 0x016AAAD0;
        /// Number of entries.
        const COUNT: u32 = 0x300;
        /// Entry size in bytes.
        const STRIDE: u32 = 0x40;
        /// Entry-method callee id.
        const METHOD: u32 = 1;
        let mut entry = lf_checker_rt::relocated(BASE);
        for _ in 0..COUNT {
            let _: u32 = lf_checker_rt::callee_thiscall!(METHOD, u32, entry);
            entry = entry.wrapping_add(STRIDE);
        }
    }
});
