// original: 0x00a53290 vehicle_add_part_auto (proposed)

/// Add a part with an automatic index: forwards to the indexed adder.
///
/// Passes its three arguments plus -1 (take the next free index) to the
/// part-adder callee, keeping `this` in ecx. Thiscall, three stack words,
/// one callee, no result.
lf_checker_rt::export!(thiscall, rw_00a53290(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const ADD: u32 = 1;
        const AUTO: u32 = 0xffff_ffff;
        lf_checker_rt::callee_thiscall!(ADD, u32, this, a0, a1, a2, AUTO);
        0
    }
});
