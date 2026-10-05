// original: 0x00a13350 dir_vector_sincos_b (proposed)
/// Fetch a direction vector from the link or synthesise it, negated first.
///
/// When the word at `this + 0x20` is non-null, copies the three words at
/// the linked object's `+0x10` to `out`. Otherwise calls the two float
/// helpers (which take the angle at `this + 0x1c` in XMM0), stores the
/// first answer sign-flipped at `out + 0` and the second at `out + 4`, with
/// a zero third word. No return value is compared (the exit register is the
/// entry value on one path). Thiscall, one stack argument.
export!(thiscall, rw_00a13350(this: u32, out: u32) -> u32 {
    unsafe {
        const ANGLE_OFF: u32 = 0x1c;
        const LINK_OFF: u32 = 0x20;
        const ROW_OFF: u32 = 0x10;
        const HELPER_A: u32 = 1;
        const HELPER_B: u32 = 2;
        const SIGN_ADDR: u32 = 0x00fe8fa0;
        let link = ((this + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 {
            for k in 0..3u32 {
                let w = ((link + ROW_OFF + 4 * k) as *const u32).read_unaligned();
                ((out + 4 * k) as *mut u32).write_unaligned(w);
            }
        } else {
            let ang = ((this + ANGLE_OFF) as *const u32).read_unaligned();
            let a = callee_cdecl!(HELPER_A, u32, ang);
            let sign = *global::<u32>(SIGN_ADDR);
            (out as *mut u32).write_unaligned(a ^ sign);
            let b = callee_cdecl!(HELPER_B, u32, ang);
            ((out + 4) as *mut u32).write_unaligned(b);
            ((out + 8) as *mut u32).write_unaligned(0);
        }
        0
    }
});
