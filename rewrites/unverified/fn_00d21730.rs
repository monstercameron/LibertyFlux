// original: 0x00D21730 find_fragment_fx_destroy_entry
/// Searches a signed-counted record array for the first record whose leading
/// 32-bit key equals `key`, returning that record's address or null. The
/// count is signed; nonpositive counts return null without reading the table.
/// Each record is 0xc bytes. The caller supplies `this` in ECX and
/// `key` on the stack; the function consumes that one stack word.
lf_checker_rt::export!(thiscall, rw_00d21730(this: u32, key: u32) -> u32 {
    const COUNT_OFFSET: u32 = 0x180c;
    const TABLE_OFFSET: u32 = 0x1810;
    const RESULT_OFFSET: u32 = 0x1810;
    const RECORD_STRIDE: u32 = 0xc;

    let count = unsafe { (this.wrapping_add(COUNT_OFFSET) as *const i32).read_unaligned() };
    if count <= 0 {
        return 0;
    }

    let mut index = 0i32;
    while index < count {
        let entry = this
            .wrapping_add(TABLE_OFFSET)
            .wrapping_add((index as u32).wrapping_mul(RECORD_STRIDE));
        let candidate = unsafe { (entry as *const u32).read_unaligned() };
        if candidate == key {
            return this
                .wrapping_add(RESULT_OFFSET)
                .wrapping_add((index as u32).wrapping_mul(RECORD_STRIDE));
        }
        index += 1;
    }
    0
});
