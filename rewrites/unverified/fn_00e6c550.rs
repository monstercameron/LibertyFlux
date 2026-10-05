// original: 0x00e6c550 veh_array_init_2x54
/// Call the element routine over 2 objects in a static array.
///
/// Passes `BASE + i * STRIDE` in ECX to the stubbed element routine
/// (thiscall/0) for `i` in `0..2`, where `BASE` is 0x0171D6C0 and
/// `STRIDE` is 0x54. Preserves ESI/EDI across the loop. Returns the
/// last call's answer in EAX. Takes no arguments.
///
/// Original: 0x00E6C550, cdecl, no arguments.
export!(cdecl, rw_00e6c550() -> u32 {
    unsafe {
        const BASE: u32 = 0x171D6C0;
        const COUNT: usize = 2;
        const STRIDE: u32 = 0x54;
        let mut ans = 0u32;
        let mut i = 0usize;
        while i < COUNT {
            ans = callee_thiscall!(1, u32, relocated(BASE + i as u32 * STRIDE));
            i += 1;
        }
        ans
    }
});
