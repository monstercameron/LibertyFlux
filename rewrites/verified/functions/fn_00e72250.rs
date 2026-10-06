// original: 0x00e72250 audio_array_teardown_16
/// Run the per-element teardown callee over 16 records of 0x20 bytes ending at 0x012f9538.
///
/// Walks the static array from the last element down to the first: the
/// original holds a descending cursor (starting at the end address) and a
/// SIGNED down-counter (starting at 15, looping while non-negative
/// after decrement), invoking the teardown callee (thiscall/0, callee id 1)
/// with each element address in ECX. The callee's answers are ignored.
/// Takes no arguments (cdecl/0); the contract compares no return channel.
export!(cdecl, rw_00e72250() -> u32 {
    unsafe {
        const END: u32 = 0x012f9538;
        const COUNT: u32 = 16;
        const STRIDE: u32 = 0x20;
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
