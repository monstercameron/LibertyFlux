// original: 0x00a3a9d0 vehicle_slots_drain (proposed)

/// Release every occupied slot of a 16-entry table.
///
/// Slots are `0x30` apart from `TABLE` to `END`. A slot whose head word is
/// non-zero is handed to the release callee (id 1, its global address as
/// the argument, answer ignored). Cdecl, no arguments, no defined return.
lf_checker_rt::export!(cdecl, rw_00a3a9d0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x012E_1EE0;
        const END: u32 = 0x012E_21E0;
        const STRIDE: u32 = 0x30;
        const RELEASE: u32 = 1;
        let mut p = TABLE;
        while p < END {
            if core::ptr::read(lf_checker_rt::global::<u32>(p)) != 0 {
                let slot = lf_checker_rt::relocated(p);
                let _: u32 = lf_checker_rt::callee_cdecl!(RELEASE, u32, slot);
            }
            p = p.wrapping_add(STRIDE);
        }
        0
    }
});
