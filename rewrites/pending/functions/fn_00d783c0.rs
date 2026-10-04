// original: 0x00d783c0 table_lookup_stride_110
/// Look up one entry in the stride-0x110 table by the index at +0x940.
export!(thiscall, rw_00d783c0(this_: u32) -> u32 {
    unsafe {
        let idx = *((this_ + 0x940) as *const u32);
        let entry = relocated(0x119F100).wrapping_add(idx.wrapping_mul(0x110));
        *(entry as *const u32)
    }
});
