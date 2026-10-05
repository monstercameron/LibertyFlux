// original: 0x00a910d0 stream_buffer_init

/// Initializes or retags a small counted buffer header.
///
/// `this` holds a buffer pointer at `+0` and two tag words at `+4`/`+6`.
/// When the tag at `+6` is already set, only `+4` is overwritten with the
/// low word of `n` and `(incoming EAX & 0xFFFF0000) | (n & 0xFFFF)` is
/// returned (the contract fixes incoming EAX to 0, so the low word of `n`;
/// Rust cannot read the upper half). Otherwise `+6` takes the low word of
/// `n`; a zero `n` stores a null buffer and returns 0, while a non-zero `n`
/// allocates `n * 4` bytes (callee 1, cdecl), stores the pointer at `+0`
/// and returns it. One call on the allocate path.
/// Original: 0x00A910D0 (thiscall, ECX + one stack word), 76 bytes.
lf_checker_rt::export!(thiscall, rw_00a910d0(this: u32, n: u32) -> u32 {
    unsafe {
        const BUF_OFF: u32 = 0x00;
        const TAG_A_OFF: u32 = 0x04;
        const TAG_B_OFF: u32 = 0x06;
        const ALLOC: u32 = 1;
        let tag_b = (this.wrapping_add(TAG_B_OFF) as *const u16).read_unaligned();
        if tag_b != 0 {
            (this.wrapping_add(TAG_A_OFF) as *mut u16).write_unaligned(n as u16);
            return (n & 0xFFFF);
        }
        (this.wrapping_add(TAG_B_OFF) as *mut u16).write_unaligned(n as u16);
        if n == 0 {
            (this.wrapping_add(TAG_A_OFF) as *mut u16).write_unaligned(0);
            (this.wrapping_add(BUF_OFF) as *mut u32).write_unaligned(0);
            return 0;
        }
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, n.wrapping_mul(4));
        (this.wrapping_add(TAG_A_OFF) as *mut u16).write_unaligned(n as u16);
        (this.wrapping_add(BUF_OFF) as *mut u32).write_unaligned(buf);
        buf
    }
});
