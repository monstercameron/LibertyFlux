// original: 0x00887EA0 stream_probe_or_tail (proposed)

/// Probe the stream tag, or tail-jump to the shared handler when idle.
///
/// When the mode word at `this + 0x20` is non-zero, forwards the first
/// argument to the tag probe entry (callee 1) and returns its answer.
/// When it is zero, tail-jumps to the shared handler (callee 2) with both
/// arguments; its answer is the answer of this function.
///
/// Original: 0x00887EA0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00887EA0(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x20;
        const PROBE: u32 = 1;
        const TAIL: u32 = 2;
        let m = ((this + MODE) as *const u32).read_unaligned();
        if m != 0 {
            lf_checker_rt::callee_thiscall!(PROBE, u32, this, arg0)
        } else {
            lf_checker_rt::callee_thiscall!(TAIL, u32, this, arg0, arg1)
        }
    }
});
