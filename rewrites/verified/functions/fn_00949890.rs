// original: 0x00949890 pool_clear_match_flags
/// Scan 16 entries with stride 0x40 and clear the flag byte at offset
/// 0x2C in every entry whose key dword matches `value`.
export!(thiscall, rw_00949890(obj: *mut u8, value: u32) -> u32 {
    unsafe {
        for i in 0..16usize {
            let e = obj.add(i * 0x40);
            if *(e as *const u32) == value {
                *e.add(0x2C) = 0;
            }
        }
        0
    }
});
