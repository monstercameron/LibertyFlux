// original: 0x00c03100 stream_field_set3 (proposed)

/// Load three fields of the streaming object from three caller slots.
///
/// `this` points to the object; `i1..i3` point at caller-owned dwords whose
/// values are stored into the fields at `+0x08`, `+0x0c` and
/// `+0x10`. Returns the third value (what the original leaves in `eax`).
///
/// Original: 0x00c03100 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c03100(this: u32, i1: u32, i2: u32, i3: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x08;
        const F1: u32 = 0x0c;
        const F2: u32 = 0x10;
        (this.wrapping_add(F0) as *mut u32).write_unaligned((i1 as *const u32).read());
        (this.wrapping_add(F1) as *mut u32).write_unaligned((i2 as *const u32).read());
        let v = (i3 as *const u32).read();
        (this.wrapping_add(F2) as *mut u32).write_unaligned(v);
        v
    }
});
