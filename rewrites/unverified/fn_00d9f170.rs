// original: 0x00D9F170 stream_recent_tag_filter
/// Scans a fixed 64-entry history backward from the current slot. It stops once an entry timestamp is older than the saturated current-time cutoff, returns zero if a recent tag difference lies within the inclusive sorted byte range, and returns one when no entry qualifies.
lf_checker_rt::export!(thiscall, rw_s675_00d9f170(this: u32, max_age: u32, low_byte: u32, high_byte: u32) -> u32 {
    const BASE_TAG_OFFSET: u32 = 4;
    const CURRENT_INDEX_OFFSET: u32 = 8;
    const HISTORY_POINTER_OFFSET: u32 = 0x0C;
    const ENTRY_STRIDE: u32 = 8;
    const HISTORY_LEN: u32 = 64;
    let base_tag = unsafe { (this.wrapping_add(BASE_TAG_OFFSET) as *const u8).read() };
    let current_index = unsafe { (this.wrapping_add(CURRENT_INDEX_OFFSET) as *const u8).read() };
    let history = unsafe { (this.wrapping_add(HISTORY_POINTER_OFFSET) as *const u32).read_unaligned() };
    let current_entry = history.wrapping_add(u32::from(current_index).wrapping_mul(ENTRY_STRIDE));
    let current_time = unsafe { (current_entry.wrapping_add(4) as *const u32).read_unaligned() };
    let cutoff = if max_age > current_time { 0 } else { current_time - max_age };
    let first = low_byte as u8;
    let second = high_byte as u8;
    let low = first.min(second);
    let high = first.max(second);
    for age in 0..HISTORY_LEN {
        let slot = current_index.wrapping_sub(age as u8) & 0x3F;
        let entry = history.wrapping_add(u32::from(slot).wrapping_mul(ENTRY_STRIDE));
        let timestamp = unsafe { (entry.wrapping_add(4) as *const u32).read_unaligned() };
        if timestamp < cutoff {
            return 1;
        }
        let category = unsafe { (entry as *const u8).read() } ^ base_tag;
        if category >= low && category <= high {
            return 0;
        }
    }
    1
});
