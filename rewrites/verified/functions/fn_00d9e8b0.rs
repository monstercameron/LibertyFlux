// original: 0x00D9E8B0 find_stream_object
/// Scans a counted pointer list, asks the object identifier helper about each entry, and returns the first pointer whose identifier equals the requested value; returns null for an empty or unsuccessful search.
lf_checker_rt::export!(thiscall, rw_s675_00d9e8b0(this: u32, target_id: u32) -> u32 {
    const TABLE_OFFSET: u32 = 0x58;
    const COUNT_OFFSET: u32 = 0x5C;
    let count = unsafe { (this.wrapping_add(COUNT_OFFSET) as *const u16).read_unaligned() } as u32;
    let entries = unsafe { (this.wrapping_add(TABLE_OFFSET) as *const u32).read_unaligned() };
    for index in 0..count {
        let entry_address = entries.wrapping_add(index.wrapping_mul(4));
        let object = unsafe { (entry_address as *const u32).read_unaligned() };
        let object_id: u32 = lf_checker_rt::callee_thiscall!(1, u32, object);
        if object_id == target_id {
            return object;
        }
    }
    0
});
