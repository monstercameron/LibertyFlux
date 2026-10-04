// original: 0x009821e0 audio_clear_entry_status
/// Clear the status dword of every entry in the table at `this+0x40`.
///
/// Reads the entry count from `this+0x30`; for each of the `count` pointers
/// zeroes the (unaligned) dword at entry+0x22. Returns the count
/// (0 when empty).
export!(thiscall, rw_009821e0(this: u32) -> u32 {
    unsafe {
        let n = *((this.wrapping_add(0x30)) as *const u32);
        if n == 0 {
            return 0;
        }
        let arr = *((this.wrapping_add(0x40)) as *const u32);
        let mut i = 0u32;
        while i < n {
            let p = *((arr.wrapping_add(i * 4)) as *const u32);
            core::ptr::write_unaligned((p.wrapping_add(0x22)) as *mut u32, 0);
            i = i.wrapping_add(1);
        }
        n
    }
});
