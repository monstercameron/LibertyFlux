// original: 0x00AF8540 veh_arrays_shift_up (proposed)

/// Make room at the front of three parallel arrays packed in one object.
///
/// Shifts the thirteen dwords at `this + 0x00` up to `this + 0x04`, the
/// thirteen words at `this + 0x38` up to `this + 0x3A`, and the thirteen
/// bytes at `this + 0x54` up to `this + 0x55`, each by one element, then
/// writes the empty marker `0xFFFFFFFF` to the freed dword at `this + 0x00`.
/// The copy runs back to front so overlapping reads see unshifted data.
///
/// Original: 0x00AF8540 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00AF8540(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 13;
        const EMPTY: u32 = 0xFFFF_FFFF;
        for k in 0..COUNT {
            let i = COUNT - 1 - k;
            let v = ((this + i * 4) as *const u32).read_unaligned();
            ((this + 4 + i * 4) as *mut u32).write_unaligned(v);
            let w = ((this + 0x38 + i * 2) as *const u16).read_unaligned();
            ((this + 0x3A + i * 2) as *mut u16).write_unaligned(w);
            let b = ((this + 0x54 + i) as *const u8).read();
            ((this + 0x55 + i) as *mut u8).write(b);
        }
        (this as *mut u32).write_unaligned(EMPTY);
        0
    }
});
