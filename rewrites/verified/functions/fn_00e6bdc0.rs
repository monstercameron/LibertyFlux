// original: 0x00e6bdc0 veh_array_init_50x280
/// Call the element routine over 50 objects in a static array.
///
/// Passes `BASE + i * STRIDE` in ECX to the stubbed element routine
/// (thiscall/0) for `i` in `0..50`, where `BASE` is 0x01713AB0 and
/// `STRIDE` is 0x280. Preserves ESI/EDI across the loop. Returns the
/// last call's answer in EAX. Takes no arguments.
///
/// Original: 0x00E6BDC0, cdecl, no arguments.
export!(cdecl, rw_00e6bdc0() -> u32 {
    unsafe {
        const BASE: u32 = 0x1713AB0;
        const COUNT: usize = 50;
        const STRIDE: u32 = 0x280;
        let mut ans = 0u32;
        let mut i = 0usize;
        while i < COUNT {
            ans = callee_thiscall!(1, u32, relocated(BASE + i as u32 * STRIDE));
            i += 1;
        }
        ans
    }
});
