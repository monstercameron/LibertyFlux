// original: 0x00B22BF0 ide_float_row_copy
/// Copy all sixteen four-byte words of a 64-byte IDE float row from `source`
/// to `this` in ascending order. Copying word-by-word preserves the original
/// forward behavior when source and destination overlap.
lf_checker_rt::export!(thiscall, rw_00b22bf0(this: u32, source: u32) -> u32 {
    unsafe {
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 { unsafe { (address as *const u32).read_unaligned() } }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) { unsafe { (address as *mut u32).write_unaligned(value) } }

        for word_index in 0..16u32 {
            let value = read_u32(source.wrapping_add(word_index * 4));
            write_u32(this.wrapping_add(word_index * 4), value);
        }
        0
    }
});
