// original: 0x009a3e50 audio_reset_and_report_capacity
/// Original 0x009a3e50 (unnamed): reset audio counters and report capacity.
///
/// Zeroes the two counter dwords, runs the four re-init helpers, then sizes
/// the report argument from the last helper's answer: the answer times 8
/// when that fits in 32 bits, else all bits set. Passes it to the reporter,
/// stores the report id in the third global, and returns it.
export!(cdecl, rw_009a3e50() -> u32 {
    unsafe {
        (relocated(0x012845C0) as *mut u32).write(0);
        (relocated(0x012845C4) as *mut u32).write(0);
    }
    callee_cdecl!(1, u32,);
    callee_cdecl!(2, u32,);
    callee_cdecl!(3, u32,);
    let cap = callee_cdecl!(4, u32,);
    let scaled = (cap as u64) * 8;
    let arg = if scaled > 0xffff_ffff {
        0xffff_ffff
    } else {
        cap.wrapping_mul(8)
    };
    let id = callee_cdecl!(5, u32, arg);
    unsafe { (relocated(0x012845CC) as *mut u32).write(id) };
    id
});
