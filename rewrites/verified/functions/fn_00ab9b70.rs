// original: 0x00ab9b70 format_indexed_byte
/// Format the table entry selected by `a0` with the byte `a1` and sink it.
///
/// Pushes `(frame_buf, format, table[a0], a1 as byte)` to the formatter
/// (callee 1), passes a second frame buffer to the sink (callee 2), then
/// runs the stack-cookie check (callee 3). The cookie dance is pure frame
/// bookkeeping below the incoming stack pointer, hence unobservable; the
/// frame-buffer addresses are skipped in the call comparison while the
/// format entry, table word and byte are compared exactly. Returns
/// nothing meaningful (the check call's answer).
lf_checker_rt::export!(cdecl, rw_00ab9b70(a0: u32, a1: u32) -> u32 {
    let mut frame = [0u32; 21];
    let base = frame.as_mut_ptr() as u32;
    // SAFETY: the table index stays inside the declared image range and
    // both buffers stay inside the local frame on every trial.
    let t = unsafe { lf_checker_rt::global::<u32>(0x0103_EE8C).offset(a0 as isize).read_unaligned() };
    let dst1 = base.wrapping_add(8);
    let _ = lf_checker_rt::callee_cdecl!(1, u32, dst1, lf_checker_rt::relocated(0x00EA_55F4), t, a1 & 0xFF);
    let dst2 = base.wrapping_add(16);
    let _ = lf_checker_rt::callee_cdecl!(2, u32, dst2);
    lf_checker_rt::callee_cdecl!(3, u32,)
});

