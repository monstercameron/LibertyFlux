// original: 0x00c037c0 stream_reg_9d_c (proposed)

/// Register a streaming event (kind `0x9d`) and hand a field to a worker.
///
/// `this` points to the record. Calls the registrar callee with
/// (`0x9d`, the shared streaming global, `a2`, `a1`, 0), rewrites `+0x03`
/// to clear bit 2 and set bit 1, then calls the worker callee with the
/// address of the field at `+0x10`, `a3` and `0x0b`. Stores zero at
/// `+0x1b`, the float `f4` at `+0x1c` and the marker `0x15` at `+0x02`.
/// Returns the worker callee's answer (what the original leaves in `eax`).
///
/// Original: 0x00c037c0 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00c037c0(this: u32, a1: u32, a2: u32, a3: u32, f4: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x9d;
        const MARKER: u8 = 0x15;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        const WORK_OP: u32 = 0x0b;
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND, g, a2, a1, 0);
        let b3 = ((this.wrapping_add(3)) as *const u8).read();
        ((this.wrapping_add(3)) as *mut u8).write((b3 & 0xfb) | 2);
        let ans = lf_checker_rt::callee_cdecl!(2, u32, this.wrapping_add(0x10), a3, WORK_OP);
        ((this.wrapping_add(0x1b)) as *mut u8).write(0);
        (this.wrapping_add(0x1c) as *mut u32).write_unaligned(f4);
        ((this.wrapping_add(2)) as *mut u8).write(MARKER);
        ans
    }
});
