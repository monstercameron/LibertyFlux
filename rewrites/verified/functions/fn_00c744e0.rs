// original: 0x00C744E0 find_model_info_pair

/// Search the object's eight-byte key/value entries for `key`. The signed
/// count at `+0xe8` controls the scan; nonpositive counts fail. On a match,
/// copy the paired value to `out` and return true in AL. On failure, leave
/// `out` untouched and return false in AL.
lf_checker_rt::export!(thiscall, rw_00c744e0(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0xe8;
        const ENTRIES: u32 = 0xc0;
        let count = ((this + COUNT) as *const u32).read_unaligned() as i32;
        for index in 0..count.max(0) as u32 {
            let entry = this.wrapping_add(ENTRIES).wrapping_add(index.wrapping_mul(8));
            if (entry as *const u32).read_unaligned() == key {
                let value = ((entry + 4) as *const u32).read_unaligned();
                (out as *mut u32).write_unaligned(value);
                return 1;
            }
        }
    }
    0
});
