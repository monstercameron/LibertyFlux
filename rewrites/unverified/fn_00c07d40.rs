// original: 0x00c07d40 stream_list_get_checked (proposed)

/// Fetch one entry from the streaming list with a bounds check.
///
/// `this` points to the list (`ITEMS` holds the entry array, `LEN` its 16-bit
/// length). Returns the `index`-th entry when `index` is below the length,
/// otherwise null. The comparison is signed: a negative index reads below the
/// array, exactly as the original does.
///
/// Original: 0x00c07d40 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c07d40(this: u32, index: u32) -> u32 {
    unsafe {
        const ITEMS: u32 = 0x08;
        const LEN: u32 = 0x0c;
        let len = (this.wrapping_add(LEN) as *const u16).read_unaligned() as u32;
        if (index as i32) < (len as i32) {
            let base = (this.wrapping_add(ITEMS) as *const u32).read_unaligned();
            (base.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned()
        } else {
            0
        }
    }
});
