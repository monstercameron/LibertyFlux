// original: 0x00b06490 store_float_1b8
/// Store one float argument into the object field at `+0x1b8`.
///
/// thiscall `(this, value)`: copies the 32-bit stack word bit-for-bit; no
/// return value (the original leaves the return register untouched).
export!(thiscall, rw_00b06490(this: u32, val: u32) -> u32 {
    unsafe {
        ((this + 0x1B8) as *mut u32).write_unaligned(val);
    }
    0 // unchecked: contract compares no return channel
});
