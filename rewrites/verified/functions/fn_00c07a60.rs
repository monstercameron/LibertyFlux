// original: 0x00c07a60 stream_table_get_checked (proposed)

/// Fetch one entry from the streaming table with a bounds check.
///
/// `this` points to the table (`TABLE` holds the entry array, `COUNT` its
/// 16-bit length). Returns the `index`-th entry when `index` is below the
/// length, otherwise null. The comparison is signed: a negative index reads
/// below the array, exactly as the original does.
///
/// Original: 0x00c07a60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c07a60(this: u32, index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x18;
        const COUNT: u32 = 0x1c;
        let count = (this.wrapping_add(COUNT) as *const u16).read_unaligned() as u32;
        if (index as i32) < (count as i32) {
            let base = (this.wrapping_add(TABLE) as *const u32).read_unaligned();
            (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
        } else {
            0
        }
    }
});
