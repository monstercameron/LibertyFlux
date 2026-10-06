// original: 0x00697750 weight_probe_setup (proposed)

/// Probe the weight table through a stack-built descriptor.
///
/// Copies the first two words of `a1` plus `a3` into a three-word frame on
/// its own stack and invokes the prober with that frame as its object. The
/// callee pops two stack words it never reads. `a2` is ignored. No result.
///
/// Original: cdecl, three stack words, caller cleans.
lf_checker_rt::export!(cdecl, rw_00697750(a1: u32, _a2: u32, a3: u32) -> u32 {
    unsafe {
        const PROBER: u32 = 1;
        let w0 = (a1 as *const u32).read_unaligned();
        let w1 = (a1 as *const u32).byte_offset(4).read_unaligned();
        let frame = [w0, w1, a3];
        // The two stack words are unread scratch the callee pops; the proof
        // skips them and snapshots the frame instead.
        let _ = lf_checker_rt::callee_thiscall!(PROBER, u32, frame.as_ptr() as u32, 0u32, 0u32);
        let _ = _a2;
        0
    }
});
