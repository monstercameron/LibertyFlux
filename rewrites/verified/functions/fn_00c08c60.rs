// original: 0x00c08c60 stream_state_set_float (proposed)

/// Store one caller float into the streaming state object.
///
/// `this` points to the object; the 4 bytes at `SLOT` are replaced by `value`,
/// copied bit for bit. The original leaves no meaningful return value (it
/// never writes `eax`), so callers must treat the result as absent.
///
/// Original: 0x00c08c60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c08c60(this: u32, value: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x10c;
        (this.wrapping_add(SLOT) as *mut u32).write_unaligned(value);
        0
    }
});
