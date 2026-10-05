// original: 0x008CBF30 stream_slot_init (proposed)

/// Initialises a streaming slot: zeroes the dword at `+0x00`, sets the flag
/// byte at `+0x04` to 1 and stores `value` at `+0x08`.
///
/// Thiscall on the slot pointer in ECX with one stack argument; pops the
/// argument (the callee pops 4 bytes); no return value.
lf_checker_rt::export!(thiscall, rw_008CBF30(this: u32, value: u32) -> u32 {
    unsafe {
        (this as *mut u32).write_unaligned(0);
        ((this + 4) as *mut u8).write(1);
        ((this + 8) as *mut u32).write_unaligned(value);
        0
    }
});
