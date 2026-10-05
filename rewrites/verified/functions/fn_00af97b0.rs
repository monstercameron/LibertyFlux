// original: 0x00AF97B0 veh_count_below_cap (proposed)

/// Test whether the live count is below its cap, both read as signed.
///
/// Compares the dword at `COUNT` against the dword at `CAP` with a signed
/// less-than (the original's `cmp` + `setl`) and returns 1 when the count
/// is below the cap.
///
/// Original: 0x00AF97B0 (cdecl, no arguments, result in AL).
lf_checker_rt::export!(cdecl, rw_00AF97B0() -> u8 {
    unsafe {
        const COUNT: u32 = 0x1600154;
        const CAP: u32 = 0x103FFA4;
        let count = (lf_checker_rt::global::<u32>(COUNT) as *const u32).read_unaligned();
        let cap = (lf_checker_rt::global::<u32>(CAP) as *const u32).read_unaligned();
        ((count as i32) < (cap as i32)) as u8
    }
});
