// original: 0x00e71620 audio_array_teardown_200
/// Run the per-element teardown callee over 512 records of 0x14 bytes ending at 0x0120f270.
///
/// Walks the static array from the last element down to the first: the
/// original holds a descending cursor (starting at the end address) and a
/// SIGNED down-counter (starting at 511, looping while non-negative
/// after decrement), invoking the teardown callee (thiscall/0, callee id 1)
/// with each element address in ECX. The callee's answers are ignored.
/// Takes no arguments (cdecl/0); the contract compares no return channel.
export!(cdecl, rw_00e71620() -> u32 {
    unsafe {
        const END: u32 = 0x0120f270;
        const COUNT: u32 = 512;
        const STRIDE: u32 = 0x14;
        let mut ptr = relocated(END);
        let mut remaining = COUNT;
        while remaining > 0 {
            ptr = ptr.wrapping_sub(STRIDE);
            lf_checker_rt::callee_thiscall!(1, u32, ptr);
            remaining -= 1;
        }
        0
    }
});
