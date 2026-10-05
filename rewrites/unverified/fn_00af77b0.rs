// original: 0x00AF77B0 veh_handles_reset_12 (proposed)

/// Reset twelve handle slots to the empty marker.
///
/// Writes `0xFFFFFFFF` to the three dwords at `this + 0x00` through
/// `this + 0x08` and the nine dwords at `this + 0x14` through `this + 0x34`.
/// The dwords at `+0x0c` and `+0x10` are left alone. No value is returned.
///
/// Original: 0x00AF77B0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF77B0(this: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0xFFFF_FFFF;
        for off in [0x00u32, 0x04, 0x08] {
            ((this + off) as *mut u32).write_unaligned(EMPTY);
        }
        for i in 0..9u32 {
            ((this + 0x14 + i * 4) as *mut u32).write_unaligned(EMPTY);
        }
        0
    }
});
