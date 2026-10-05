// original: 0x0061CF30 net_copy_session_state

/// Copy a session snapshot into this endpoint.
///
/// Runs the shared block copier over `a0`, copies the two status words
/// at `a0+0x38` to `this+0x38`, copies the four header words at `a1` to
/// `this+0x40`, runs the copier over (`this+0x50`, `a1+0x10`), then
/// moves 0x10E dwords from `a2` to `this+0x88`. Returns the copier's
/// last answer.
/// Original: 0x0061CF30 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_0061CF30(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const COPY_CALLEE: u32 = 1;
        const BULK_WORDS: usize = 0x10E;
        lf_checker_rt::callee_thiscall!(COPY_CALLEE, u32, this, a0);
        ((this + 0x38) as *mut u32)
            .write_unaligned(((a0 + 0x38) as *const u32).read_unaligned());
        ((this + 0x3C) as *mut u32)
            .write_unaligned(((a0 + 0x3C) as *const u32).read_unaligned());
        for i in 0..4u32 {
            ((this + 0x40 + i * 4) as *mut u32)
                .write_unaligned(((a1 + i * 4) as *const u32).read_unaligned());
        }
        let answer = lf_checker_rt::callee_thiscall!(COPY_CALLEE, u32, this + 0x50, a1 + 0x10);
        for i in 0..BULK_WORDS {
            ((this + 0x88 + (i * 4) as u32) as *mut u32)
                .write_unaligned(((a2 + (i * 4) as u32) as *const u32).read_unaligned());
        }
        answer
    }
});
