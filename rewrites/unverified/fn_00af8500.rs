// original: 0x00AF8500 veh_arrays_shift_down (proposed)

/// Drop the first entry of three parallel arrays packed in one object.
///
/// Shifts the thirteen dwords at `this + 0x04` down to `this + 0x00`, the
/// thirteen words at `this + 0x3A` down to `this + 0x38`, and the thirteen
/// bytes at `this + 0x55` down to `this + 0x54`, each by one element, then
/// writes the empty marker `0xFFFFFFFF` to the freed dword at `this + 0x34`.
/// The copy runs front to back so overlapping reads see unshifted data.
///
/// Original: 0x00AF8500 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF8500(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 13;
        const EMPTY: u32 = 0xFFFF_FFFF;
        for i in 0..COUNT {
            let v = ((this + 4 + i * 4) as *const u32).read_unaligned();
            ((this + i * 4) as *mut u32).write_unaligned(v);
            let w = ((this + 0x3A + i * 2) as *const u16).read_unaligned();
            ((this + 0x38 + i * 2) as *mut u16).write_unaligned(w);
            let b = ((this + 0x55 + i) as *const u8).read();
            ((this + 0x54 + i) as *mut u8).write(b);
        }
        ((this + 0x34) as *mut u32).write_unaligned(EMPTY);
        0
    }
});
