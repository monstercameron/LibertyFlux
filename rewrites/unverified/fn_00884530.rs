// original: 0x00884530 stream_pair_set (proposed)
/// Store two values into a streaming object: `first` at `+0x1c`, `second` at `+0x08`.
///
/// No return value is produced.
///
/// Original: thiscall, two stack arguments, callee cleans 8.
lf_checker_rt::export!(thiscall, rw_00884530(this: u32, first: u32, second: u32) -> u32 {
    unsafe {
        ((this + 0x1c) as *mut u32).write_unaligned(first);
        ((this + 0x08) as *mut u32).write_unaligned(second);
        0
    }
});
