// original: 0x00D9EEC0 stream_find_record
/// Searches a counted array of eight-byte entries for the first matching key and returns its paired value. If no entry matches, either sentinel argument produces 0x3E; otherwise the result is -1.
lf_checker_rt::export!(thiscall, rw_s675_00d9eec0(this: u32, key: u32, fallback_key: u32) -> u32 {
    const COUNT_OFFSET: u32 = 0x54;
    const ARRAY_OFFSET: u32 = 0x50;
    const ENTRY_STRIDE: u32 = 8;
    const SENTINEL_KEY: u32 = 0x3A;
    const SENTINEL_RESULT: u32 = 0x3E;
    let count = unsafe { (this.wrapping_add(COUNT_OFFSET) as *const u16).read_unaligned() as u32 };
    let entries = unsafe { (this.wrapping_add(ARRAY_OFFSET) as *const u32).read_unaligned() };
    for index in 0..count {
        let entry = entries.wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
        if unsafe { (entry as *const u32).read_unaligned() } == key {
            return unsafe { (entry.wrapping_add(4) as *const u32).read_unaligned() };
        }
    }
    if key == SENTINEL_KEY || fallback_key == SENTINEL_KEY { SENTINEL_RESULT } else { u32::MAX }
});
