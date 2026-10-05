// original: 0x00ABF0B0 stream_tabled_float (proposed)

/// Return the float at index `index` of the streaming float table.
///
/// The original loads `[index * 4 + TABLE]` onto the x87 stack with `fld` and
/// returns it in ST0 (cdecl, one stack word). The table lives in zero-filled
/// game memory; the rewrite reads it through the relocated address.
lf_checker_rt::export!(cdecl, rw_00ABF0B0(index: u32) -> f32 {
    unsafe {
        const TABLE: u32 = 0x0150E258;
        let addr = lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4));
        (addr as *const f32).read_unaligned()
    }
});
