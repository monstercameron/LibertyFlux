// original: 0x00c03740 stream_reg_9d_a (proposed)

/// Register a streaming event (kind `0x9d`) and forward one argument.
///
/// `this` points to the record. Calls the registrar callee with
/// (`0x9d`, the shared streaming global, `a2`, `a1`, 0), sets bits `0x06`
/// of `+0x03`, pushes `a3` through the follow-up callee and writes the
/// marker `0x15` at `+0x02`. Returns the follow-up callee's answer (what
/// the original leaves in `eax`).
///
/// Original: 0x00c03740 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00c03740(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x9d;
        const MARKER: u8 = 0x15;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND, g, a2, a1, 0);
        let b3 = ((this.wrapping_add(3)) as *const u8).read();
        ((this.wrapping_add(3)) as *mut u8).write(b3 | 6);
        let ans = lf_checker_rt::callee_thiscall!(2, u32, this, a3);
        ((this.wrapping_add(2)) as *mut u8).write(MARKER);
        ans
    }
});
