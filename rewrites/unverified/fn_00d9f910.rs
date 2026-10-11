// original: 0x00D9F910 stream_has_record_key
/// Returns one when a counted array of 32-bit keys contains the requested key, and zero when the list is empty or no key matches.
lf_checker_rt::export!(thiscall, rw_s675_00d9f910(this: u32, requested_key: u32) -> u32 {
    const COUNT_OFFSET: u32 = 0x3C;
    const ARRAY_OFFSET: u32 = 0x38;
    let count = unsafe { (this.wrapping_add(COUNT_OFFSET) as *const u16).read_unaligned() as u32 };
    let entries = unsafe { (this.wrapping_add(ARRAY_OFFSET) as *const u32).read_unaligned() };
    for index in 0..count {
        if unsafe { (entries.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() } == requested_key {
            return 1;
        }
    }
    0
});
