// original: 0x0061DC50 net_bitread_single

/// Run one framed bit-read, reporting the bit count when asked.
///
/// Builds a six-word frame (`BUF`, 0, 0x1CA0, 0, `BITS`, 0) and runs it
/// with `this` through the bit reader. `buf` is recorded in the frame,
/// the middle word is popped but never loaded, and `out` is an optional
/// out-pointer for the consumed bits as bytes (`(BITS+7)>>3`), or null
/// to skip it; a rejected read stores 0 through a non-null `out`.
/// Always returns the reader's answer. (The original folds an
/// uninitialized-stack byte into a dead frame slot; the rewrite omits it.)
/// Original: 0x0061DC50 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061DC50(this: u32, buf: u32, _unused: u32, out: u32) -> u32 {
    unsafe {
        const READ_CALLEE: u32 = 1;
        const FRAME_SIZE: u32 = 0x1CA0;
        const BITS_SLOT: usize = 4;
        let mut frame = [0u32; 6];
        frame[0] = buf;
        frame[2] = FRAME_SIZE;
        let base = frame.as_mut_ptr() as u32;
        let answer = lf_checker_rt::callee_thiscall!(READ_CALLEE, u32, this, base);
        if out == 0 {
            return answer;
        }
        if answer & 0xFF == 0 {
            (out as *mut u32).write_unaligned(0);
            return answer;
        }
        let bytes = ((frame[BITS_SLOT].wrapping_add(7)) as i32 >> 3) as u32;
        (out as *mut u32).write_unaligned(bytes);
        answer
    }
});
