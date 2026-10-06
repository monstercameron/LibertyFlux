// original: 0x00e72270 audio_array_teardown_4
/// Run the per-element teardown callee over 4 records of 0x10 bytes ending at 0x012fa738.
///
/// Walks the static array from the last element down to the first: the
/// original holds a descending cursor (starting at the end address) and a
/// SIGNED down-counter (starting at 3, looping while non-negative
/// after decrement), invoking the teardown callee (thiscall/0, callee id 1)
/// with each element address in ECX. The callee's answers are ignored.
/// Takes no arguments (cdecl/0); the contract compares no return channel.
export!(cdecl, rw_00e72270() -> u32 {
    unsafe {
        const END: u32 = 0x012fa738;
        const COUNT: u32 = 4;
        const STRIDE: u32 = 0x10;
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
