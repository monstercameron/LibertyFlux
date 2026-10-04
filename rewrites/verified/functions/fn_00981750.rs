// original: 0x00981750 audio_free_guarded_record_array
/// Original 0x00981750 (unnamed): free an array with per-record guards.
///
/// For each of `count` records (stride 0x160) frees the pointer at +0x34 when
/// the flag word at +0x3a is set, and the pointer at +0x2c when the flag word
/// at +0x32 is set; then frees the array base. Returns the last answer.
export!(stdcall, rw_00981750(base: u32, count: i32) -> u32 {
    let mut ans = 0u32;
    if count > 0 {
        let mut rec = base.wrapping_add(0x32);
        for _ in 0..count {
            unsafe {
                if ((rec + 8) as *const u16).read() != 0 {
                    ans = callee_cdecl!(1, u32, ((rec + 2) as *const u32).read());
                }
                if (rec as *const u16).read() != 0 {
                    ans = callee_cdecl!(1, u32, ((rec - 6) as *const u32).read());
                }
            }
            rec = rec.wrapping_add(0x160);
        }
    }
    ans = callee_cdecl!(1, u32, base);
    ans
});
