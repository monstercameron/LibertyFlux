// original: 0x00e6c520 veh_array_init_4xb0
/// Call the element routine over 4 objects in a static array.
///
/// Passes `BASE + i * STRIDE` in ECX to the stubbed element routine
/// (thiscall/0) for `i` in `0..4`, where `BASE` is 0x0171CB10 and
/// `STRIDE` is 0xB0. Preserves ESI/EDI across the loop. Returns the
/// last call's answer in EAX. Takes no arguments.
///
/// Original: 0x00E6C520, cdecl, no arguments.
export!(cdecl, rw_00e6c520() -> u32 {
    unsafe {
        const BASE: u32 = 0x171CB10;
        const COUNT: usize = 4;
        const STRIDE: u32 = 0xB0;
        let mut ans = 0u32;
        let mut i = 0usize;
        while i < COUNT {
            ans = callee_thiscall!(1, u32, relocated(BASE + i as u32 * STRIDE));
            i += 1;
        }
        ans
    }
});
