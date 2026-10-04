// original: 0x00981710 audio_free_record_array
/// Original 0x00981710 (unnamed): free an array of fixed-size records.
///
/// Frees the pointer stored at offset +0x40 of each of `count` records
/// (stride 0x360), zeroes the two dwords there, then frees the array base
/// itself. Returns the last free-call answer (checker compares it; the real
/// callee returns void).
export!(stdcall, rw_00981710(base: u32, count: i32) -> u32 {
    let mut ans = 0u32;
    if count > 0 {
        let mut slot = base.wrapping_add(0x40);
        for _ in 0..count {
            unsafe {
                ans = callee_cdecl!(1, u32, (slot as *const u32).read());
                (slot as *mut u32).write(0);
                (slot as *mut u32).add(1).write(0);
            }
            slot = slot.wrapping_add(0x360);
        }
    }
    ans = callee_cdecl!(1, u32, base);
    ans
});
