// original: 0x00C8A590 audio_voice_select_and_spatialize (proposed)

/// Pick an active voice slot from a table and run two spatialisation passes.
///
/// `obj` points to an object whose word at `+0x20` is a ten-float vector
/// (x/y/z triples at `+0x00/+0x04/+0x08`, `+0x10/+0x14/+0x18` and
/// `+0x20/+0x24/+0x28`). `table` points to rows of 640 bytes. `index`
/// selects row group `5 * index`: the bytes at row offsets `0`, `0x80`,
/// `0x100`, `0x180`, `0x200` of that group are the five slot flags. The
/// first and the last non-zero slot are remembered; when every flag is zero
/// the function returns 0 and calls nothing.
///
/// Otherwise two passes run (pass 0 with coefficient -1.0, pass 1 with +1.0).
/// Each pass forms three outputs from the vector:
/// `out[i] = (v[i] * c + v[i+3] * 0) + v[i+6] * 0`, keeping the original's
/// exact operation order (the `* 0` terms still set the sign of zero and
/// propagate NaNs). Pass 0 works on the first slot, pass 1 on the last.
/// With the row pointer `table + ((5 * index + slot) << 7)`:
/// - when the low byte of `flag_a` is non-zero, callee 1 runs with
///   `(obj, index + bias, pass, row, &outs, tag)`;
/// - when the low byte of `flag_b` is non-zero, callee 2 runs with
///   `(obj, index + bias2, row, pass == 1)`;
/// - when the low byte of `tag` is also non-zero and the pass's slot is the
///   expected one (slot 0 on pass 0, slot 4 on pass 1), callee 3 runs with
///   `(obj, index + bias2, pass, row)`.
///
/// All three callees are thiscall with the same global object. The function
/// returns 1 in `al`. Incoming `ecx` is ignored (forced to -1). The slot
/// check `slot == -1` before the calls can never fire (an all-zero table
/// returned early) and is kept for fidelity.
///
/// Float evaluation decides operand order by a branch (see `fadd`): the
/// destination register decides the sign of a NaN result, and the compiler
/// swaps the operands of scalar operations even across call boundaries, so
/// both-NaN pairs forward the destination operand explicitly while every
/// other case runs as a plain operation whose order is irrelevant.
///
/// Original: 0x00C8A590 (thiscall shape with six stack words, callee pops
/// 0x18; the upper bytes of the flag words and the incoming `ecx` are not
/// read).
lf_checker_rt::export!(thiscall, rw_00C8A590(
    _this: u32,
    obj: u32,
    table: u32,
    index: u32,
    tag: u32,
    flag_a: u32,
    flag_b: u32,
) -> u32 {
    unsafe {
        const VEC_OFF: u32 = 0x20;
        const ROW_SHIFT: u32 = 7;
        const GROUP: u32 = 5;
        const GLOBAL_OBJ: u32 = 0x13B0EB0;
        const CALLEE_MIX: u32 = 1;
        const CALLEE_POST: u32 = 2;
        const CALLEE_TAIL: u32 = 3;
        const NO_SLOT: u32 = 0xFFFF_FFFF;
        const BIAS2: [u32; 2] = [0x18, 0x19];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        // Operand order is decided by a branch, not by the compiler: the
        // destination register decides a NaN result's sign, and the
        // compiler swaps the operands of scalar float operations even
        // across call boundaries. Order matters only when both operands
        // are NaN, so those pairs forward the destination operand
        // explicitly and everything else runs as a plain operation whose
        // order is irrelevant. SNaN cannot occur (audited: no SNaN in any
        // input pool, and no operation here creates one).
        type F = u32; // bits of one f32
        #[inline(always)]
        fn is_nan_bits(x: u32) -> bool {
            x & 0x7F800000 == 0x7F800000 && x & 0x007FFFFF != 0
        }
        #[inline(never)]
        fn fadd(a: F, b: F) -> F {
            if is_nan_bits(a) && is_nan_bits(b) {
                a | 0x00400000
            } else {
                unsafe {
                    let va = core::arch::x86::_mm_set_ss(f32::from_bits(a));
                    let vb = core::arch::x86::_mm_set_ss(f32::from_bits(b));
                    let mut o = 0f32;
                    core::arch::x86::_mm_store_ss(
                        &mut o,
                        core::arch::x86::_mm_add_ss(va, vb),
                    );
                    o.to_bits()
                }
            }
        }
        #[inline(never)]
        fn fmul(a: F, b: F) -> F {
            if is_nan_bits(a) && is_nan_bits(b) {
                a | 0x00400000
            } else {
                unsafe {
                    let va = core::arch::x86::_mm_set_ss(f32::from_bits(a));
                    let vb = core::arch::x86::_mm_set_ss(f32::from_bits(b));
                    let mut o = 0f32;
                    core::arch::x86::_mm_store_ss(
                        &mut o,
                        core::arch::x86::_mm_mul_ss(va, vb),
                    );
                    o.to_bits()
                }
            }
        }

        // Phase 1: first and last non-zero slot flag in this index's group.
        let group = index.wrapping_mul(GROUP);
        let mut first = NO_SLOT;
        let mut last = NO_SLOT;
        let mut k = 0u32;
        while k < 5 {
            let flag = rd8(table.wrapping_add(group.wrapping_add(k) << ROW_SHIFT));
            if flag != 0 {
                if first == NO_SLOT {
                    first = k;
                }
                last = k;
            }
            k += 1;
        }
        if first == NO_SLOT {
            return 0;
        }

        // Phase 2: one pass per slot (first, then last).
        let vec = rd32(obj.wrapping_add(VEC_OFF));
        let zero = 0u32;
        let mut pass = 0u32;
        while pass < 2 {
            let c = if pass == 0 { 0xBF800000u32 } else { 0x3F800000u32 };
            let sel = if pass == 0 { first } else { last };
            let bias = if pass == 0 { 0x0Eu32 } else { 0x13u32 };
            let bias2 = BIAS2[pass as usize];
            // out[i] = (v[i] * c + v[i+3] * 0) + v[i+6] * 0, in order.
            let o0 = fadd(
                fadd(fmul(rd32(vec), c), fmul(rd32(vec.wrapping_add(0x10)), zero)),
                fmul(rd32(vec.wrapping_add(0x20)), zero),
            );
            let o1 = fadd(
                fadd(
                    fmul(rd32(vec.wrapping_add(0x04)), c),
                    fmul(rd32(vec.wrapping_add(0x14)), zero),
                ),
                fmul(rd32(vec.wrapping_add(0x24)), zero),
            );
            let o2 = fadd(
                fadd(
                    fmul(rd32(vec.wrapping_add(0x08)), c),
                    fmul(rd32(vec.wrapping_add(0x18)), zero),
                ),
                fmul(rd32(vec.wrapping_add(0x28)), zero),
            );
            let outs = [f32::from_bits(o0), f32::from_bits(o1), f32::from_bits(o2)];
            let row = table.wrapping_add(group.wrapping_add(sel) << ROW_SHIFT);
            if sel != NO_SLOT {
                if (flag_a & 0xFF) != 0 {
                    lf_checker_rt::callee_thiscall!(
                        CALLEE_MIX,
                        u32,
                        lf_checker_rt::relocated(GLOBAL_OBJ),
                        obj,
                        index.wrapping_add(bias),
                        pass,
                        row,
                        outs.as_ptr() as u32,
                        tag
                    );
                }
                if (flag_b & 0xFF) != 0 {
                    lf_checker_rt::callee_thiscall!(
                        CALLEE_POST,
                        u32,
                        lf_checker_rt::relocated(GLOBAL_OBJ),
                        obj,
                        index.wrapping_add(bias2),
                        row,
                        (pass == 1) as u32
                    );
                    if (tag & 0xFF) != 0
                        && ((pass == 0 && sel == 0) || (pass == 1 && sel == 4))
                    {
                        lf_checker_rt::callee_thiscall!(
                            CALLEE_TAIL,
                            u32,
                            lf_checker_rt::relocated(GLOBAL_OBJ),
                            obj,
                            index.wrapping_add(bias2),
                            pass,
                            row
                        );
                    }
                }
            }
            pass += 1;
        }
        1
    }
});
