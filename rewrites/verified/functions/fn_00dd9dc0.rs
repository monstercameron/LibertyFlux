// original: 0x00dd9dc0 UIBasicClip::vf138

/// Forward a label to the clip's text sink, prefixing it with the clip's title
/// when titling is enabled.
///
/// `this` is the clip; `label` points at a NUL-terminated byte string and the
/// low byte of `flags` enables titling (high bytes ignored). The sink object
/// lives at `+SINK`, the title source at `+TITLE_SRC`.
/// - When the flag byte is zero, or the sink's virtual slot `TITLE_CHECK`
///   (thiscall, no stack arguments) answers null, the label is forwarded
///   as-is: the sink's virtual slot `SUBMIT` (thiscall, two stack words) is
///   called with (`label`, 0).
/// - Otherwise a 256-byte buffer is zeroed, the title string (answered by the
///   title source's `TITLE_CHECK` slot) is copied in, and the label is
///   appended when the combined length stays below 256 bytes
///   (`256 - title_len <= label_len` skips the append, unsigned 32-bit);
///   then `SUBMIT` is called with (buffer, 0).
///
/// The function ends with the CRT security-cookie check (callee 9, argument
/// skipped: it mixes the cookie with the stack pointer, which legitimately
/// differs between the sides).
///
/// Inputs are always NUL-terminated within bounds; an unterminated title or
/// label would smash the original's stack where the rewrite faults, so such
/// inputs are excluded from the proof.
///
/// Original: 0x00dd9dc0 (thiscall, two stack words, the callee pops 8 bytes, no return value).
lf_checker_rt::export!(thiscall, rw_00dd9dc0(this: u32, label: u32, flags: u32) -> u32 {
    unsafe {
        const SINK: u32 = 0x1ec;
        const TITLE_SRC: u32 = 0x1e8;
        const TITLE_CHECK: u32 = 0x20c;
        const SUBMIT: u32 = 0x1e0;
        const BUF_LEN: u32 = 0x100;
        const COOKIE_CALLEE: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn vcall0(child: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(child) + slot) as usize);
                f(child)
            }
        }
        #[inline(always)]
        unsafe fn vcall2(child: u32, slot: u32, a: u32, b: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(child) + slot) as usize);
                f(child, a, b)
            }
        }
        #[inline(always)]
        unsafe fn strlen(s: u32) -> u32 {
            unsafe {
                let mut n = 0u32;
                while rd8(s + n) != 0 {
                    n += 1;
                }
                n
            }
        }

        let sink = rd32(this + SINK);
        if (flags as u8) == 0 || vcall0(sink, TITLE_CHECK) == 0 {
            vcall2(sink, SUBMIT, label, 0);
        } else {
            let mut buf = [0u8; 256];
            let title = vcall0(rd32(this + TITLE_SRC), TITLE_CHECK);
            let mut i = 0usize;
            loop {
                let b = rd8(title + i as u32);
                buf[i] = b;
                i += 1;
                if b == 0 {
                    break;
                }
            }
            let title_len = (i as u32) - 1;
            let label_len = strlen(label);
            if !(BUF_LEN.wrapping_sub(title_len) <= label_len) {
                let mut j = 0u32;
                loop {
                    let b = rd8(label + j);
                    buf[(title_len + j) as usize] = b;
                    j += 1;
                    if b == 0 {
                        break;
                    }
                }
            }
            vcall2(sink, SUBMIT, buf.as_mut_ptr() as u32, 0);
        }
        lf_checker_rt::callee_thiscall!(COOKIE_CALLEE, u32, 0);
        0
    }
});
