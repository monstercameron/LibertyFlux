// original: 0x00C743B0 append_model_info_pair

/// Append a key/value pair to the object's five-entry table while its signed
/// count is below five. The key is written at `+0xc0 + count*8`, the value at
/// the following dword, and the count at `+0xe8` is incremented. At capacity,
/// the object is left unchanged.
lf_checker_rt::export!(thiscall, rw_00c743b0(this: u32, key: u32, value: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0xe8;
        const ENTRIES: u32 = 0xc0;
        const CAPACITY: i32 = 5;
        let count_address = this + COUNT;
        let count = (count_address as *const u32).read_unaligned();
        if (count as i32) < CAPACITY {
            let entry = this.wrapping_add(ENTRIES).wrapping_add(count.wrapping_mul(8));
            ((entry + 4) as *mut u32).write_unaligned(value);
            (entry as *mut u32).write_unaligned(key);
            (count_address as *mut u32).write_unaligned(count.wrapping_add(1));
        }
    }
    0
});
