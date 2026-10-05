// original: 0x00887380 stream_min_store (proposed)

/// Clamp a caller-supplied limit to the configured span and store it.
///
/// `this` points to the stream-slot state: the span is the word at `+0x18`
/// minus the word at `+0x1c`. The stored word at `+0x0c` is `limit` when
/// `limit` is below the span, otherwise the span. The original never writes
/// `eax`, so it has no meaningful return value.
///
/// Original: 0x00887380 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00887380(this: u32, limit: u32) -> u32 {
    unsafe {
        const RANGE_END: u32 = 0x18;
        const RANGE_START: u32 = 0x1c;
        const CLAMPED: u32 = 0x0c;
        let span = ((this + RANGE_END) as *const u32)
            .read_unaligned()
            .wrapping_sub(((this + RANGE_START) as *const u32).read_unaligned());
        let v = if limit < span { limit } else { span };
        ((this + CLAMPED) as *mut u32).write_unaligned(v);
        0
    }
});
