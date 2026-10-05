// original: 0x00c25250 cam_range_store (proposed)
/// Store a float range (low, high) into row `index` of the camera parameter
/// table: `obj + index*4 + 0x198` gets `lo`, `obj + index*4 + 0x1c0` gets
/// `hi`. Pure 4-byte moves, no return value.
///
/// Original: 0x00c25250 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c25250(obj: u32, index: u32, lo: u32, hi: u32) -> u32 {
    unsafe {
        const LO_OFF: u32 = 0x198;
        const HI_OFF: u32 = 0x1c0;
        let row = obj.wrapping_add(index.wrapping_mul(4));
        (row.wrapping_add(LO_OFF) as *mut u32).write_unaligned(lo);
        (row.wrapping_add(HI_OFF) as *mut u32).write_unaligned(hi);
        0
    }
});
