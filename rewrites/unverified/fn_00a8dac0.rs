// original: 0x00a8dac0 pool_free_array (proposed)

/// Notify then free every live entry of an array, then free the array.
///
/// `arr` points to `count` entries of 0xA0 bytes. Each entry is offered to
/// the notify callee with entry+8 as the object; when the entry's 16-bit
/// flag at +6 is non-zero, the pointer at the entry base is handed to the
/// free callee. A non-positive count skips the loop. The array itself is
/// always freed last. Returns whatever the final free returns, as the
/// original leaves it in eax.
///
/// Original: 0x00A8DAC0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00a8dac0(arr: u32, count: u32) -> u32 {
    unsafe {
        const CALLEE_NOTIFY: u32 = 1;
        const CALLEE_FREE: u32 = 2;
        const ENTRY_SIZE: u32 = 0xa0;
        const NOTIFY_THIS: u32 = 8;
        const FLAG: u32 = 6;
        let n = count as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let entry =
                    arr.wrapping_add((i as u32).wrapping_mul(ENTRY_SIZE));
                lf_checker_rt::callee_thiscall!(
                    CALLEE_NOTIFY,
                    u32,
                    entry.wrapping_add(NOTIFY_THIS)
                );
                let flag =
                    ((entry + FLAG) as *const u16).read_unaligned();
                if flag != 0 {
                    let ptr = (entry as *const u32).read_unaligned();
                    lf_checker_rt::callee_cdecl!(CALLEE_FREE, u32, ptr);
                }
                i += 1;
            }
        }
        lf_checker_rt::callee_cdecl!(CALLEE_FREE, u32, arr)
    }
});
