// original: 0x00c03050 stream_field_get3 (proposed)

/// Copy three fields out of the streaming object into three caller slots.
///
/// `this` points to the object; `o1..o3` point at caller-owned dwords that
/// receive the fields at `+0x08`, `+0x0c` and `+0x10`.
/// Returns the third slot pointer (what the original leaves in `eax`).
///
/// Original: 0x00c03050 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c03050(this: u32, o1: u32, o2: u32, o3: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x08;
        const F1: u32 = 0x0c;
        const F2: u32 = 0x10;
        (o1 as *mut u32).write_unaligned((this.wrapping_add(F0) as *const u32).read_unaligned());
        (o2 as *mut u32).write_unaligned((this.wrapping_add(F1) as *const u32).read_unaligned());
        (o3 as *mut u32).write_unaligned((this.wrapping_add(F2) as *const u32).read_unaligned());
        o3
    }
});
