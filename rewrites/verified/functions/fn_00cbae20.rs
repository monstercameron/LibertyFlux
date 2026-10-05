// original: 0x00cbae20 CTaskSimpleSlideToCoord_poll_state (proposed)

/// Poll two slide-task state words, record each helper's verdict in a flag
/// word, and run a proximity re-check when the second verdict is unclear.
///
/// `this` points to the task object. `STATE_A` (+0xac) and `STATE_B` (+0xb0)
/// hold handles polled through two cdecl helpers; `FLAGS` (+0xb4) collects
/// the outcomes. The first stack argument is never read; the second (`arg2`)
/// points to a record whose float at +8 is used only by the proximity check.
///
/// Behaviour in order:
/// - If either state handle is zero, return 0 without calling anything.
/// - Call helper 1 with (`STATE_A`, 0). A verdict of 2 or 3 also returns 0.
///   Otherwise `STATE_A` is cleared and flag bit `FLAG_A_OK` (0x10) is set to
///   whether the verdict was 1.
/// - Call helper 2 with (`STATE_B`, out), where `out` is a scratch buffer.
///   A verdict of 2 or 3 sets `FLAG_B_OK` (0x20), runs the finish step
///   (callee 3, thiscall on `this`) and returns 1. Otherwise `STATE_B` is
///   cleared and `FLAG_B_OK` is set to whether the verdict was 1.
/// - When `FLAG_B_OK` ends up clear, the proximity check runs: with `unit`
///   read from the image's float constant (1.0) and `v` the float at
///   `arg2 + 8`, it computes `dist = |out_value - (v - unit)|` in that
///   operand order and sets `FLAG_B_OK` unless `unit > dist` (ordered
///   comparison: NaN never counts as above). Return 1.
///
/// The two flag updates are single-bit sets expressed in the original as
/// xor-mask-xor sequences; only the final flag word is observable.
/// Returns 0 or 1 in `al` (upper bytes of `eax` are leftovers).
///
/// Original: 0x00cbae20 (thiscall, two stack words, the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_00cbae20(this: u32, _a1: u32, arg2: u32) -> u32 {
    unsafe {
        const STATE_A: u32 = 0xac;
        const STATE_B: u32 = 0xb0;
        const FLAGS: u32 = 0xb4;
        const FLAG_A_OK: u32 = 0x10;
        const FLAG_B_OK: u32 = 0x20;
        const ARG2_VALUE: u32 = 8;
        const UNIT_ADDR: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fabs(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & 0x7fff_ffff)
        }

        let state_a = rd32(this + STATE_A);
        if state_a == 0 {
            return 0;
        }
        let state_b = rd32(this + STATE_B);
        if state_b == 0 {
            return 0;
        }
        let r1 = lf_checker_rt::callee_cdecl!(1, u32, state_a, 0u32);
        if r1 == 2 || r1 == 3 {
            return 0;
        }
        wr32(this + STATE_A, 0);
        let flags = rd32(this + FLAGS);
        if r1 == 1 {
            wr32(this + FLAGS, flags | FLAG_A_OK);
        } else {
            wr32(this + FLAGS, flags & !FLAG_A_OK);
        }
        let mut out = [0u32; 3];
        let r2 = lf_checker_rt::callee_cdecl!(2, u32, state_b, out.as_mut_ptr() as u32);
        if r2 == 2 || r2 == 3 {
            wr32(this + FLAGS, rd32(this + FLAGS) | FLAG_B_OK);
            lf_checker_rt::callee_thiscall!(3, u32, this);
            return 1;
        }
        wr32(this + STATE_B, 0);
        let flags = rd32(this + FLAGS);
        if r2 == 1 {
            wr32(this + FLAGS, flags | FLAG_B_OK);
        } else {
            wr32(this + FLAGS, flags & !FLAG_B_OK);
        }
        let flags = rd32(this + FLAGS);
        if flags & FLAG_B_OK == 0 {
            let unit = f32::from_bits(rd32(lf_checker_rt::relocated(UNIT_ADDR)));
            let v = f32::from_bits(rd32(arg2 + ARG2_VALUE));
            let out_value = f32::from_bits(out[2]);
            let t0 = sub(v, unit);
            let dist = fabs(sub(out_value, t0));
            if !(unit > dist) {
                wr32(this + FLAGS, flags | FLAG_B_OK);
            }
        }
        1
    }
});
