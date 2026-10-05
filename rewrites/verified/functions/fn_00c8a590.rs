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
/// Float evaluation uses scalar SSE intrinsics with the original's exact
/// instruction form and operand order (see the `load`/`mul`/`add` helpers):
/// the destination register decides the sign of a NaN result, and on this
/// machine a memory source operand propagates NaN signs differently from a
/// register source, so both are pinned.
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
        // Scalar float ops with the original's exact instruction form and
        // operand order: one mulss/addss each, first argument as the
        // destination, all arithmetic in register form. Every float enters
        // through its bits passed to `black_box`: the barrier hides the
        // values (so LLVM cannot fold the known coefficient -1.0 into the
        // arithmetic, which it otherwise does, turning an add into a
        // subtraction) and routes them through general registers. A value
        // that reaches an arithmetic instruction straight from a load is
        // fused into a memory source operand by the backend, and on this
        // machine a memory source propagates NaN signs differently from a
        // register source (seen on `inf * +0`).
        type V = core::arch::x86::__m128;
        #[inline(always)]
        unsafe fn splat_bits(b: u32) -> V {
            unsafe {
                core::arch::x86::_mm_set_ss(f32::from_bits(core::hint::black_box(b)))
            }
        }
        #[inline(always)]
        unsafe fn load(a: u32) -> V {
            unsafe { splat_bits((a as *const u32).read_unaligned()) }
        }
        // The arithmetic sits behind call boundaries on purpose: with the
        // operand trees visible, LLVM swaps the operands of these scalar
        // operations (they lower to plain commutative fadd/fmul), and the
        // destination decides the sign of a NaN result. Opaque call operands
        // cannot be reordered; the DLL inspection below the contract
        // confirms the emitted order.
        #[inline(never)]
        fn mul(a: V, b: V) -> V {
            unsafe { core::arch::x86::_mm_mul_ss(a, b) }
        }
        #[inline(never)]
        fn add(a: V, b: V) -> V {
            unsafe { core::arch::x86::_mm_add_ss(a, b) }
        }
        #[inline(always)]
        unsafe fn store(v: V) -> f32 {
            unsafe {
                let mut r = 0f32;
                core::arch::x86::_mm_store_ss(&mut r, v);
                r
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
        let zero = splat_bits(0);
        let mut pass = 0u32;
        while pass < 2 {
            let c = splat_bits(if pass == 0 { 0xBF800000u32 } else { 0x3F800000u32 });
            let sel = if pass == 0 { first } else { last };
            let bias = if pass == 0 { 0x0Eu32 } else { 0x13u32 };
            let bias2 = BIAS2[pass as usize];
            // out[i] = (v[i] * c + v[i+3] * 0) + v[i+6] * 0, in order.
            let o0 = store(add(
                add(mul(load(vec), c), mul(load(vec.wrapping_add(0x10)), zero)),
                mul(load(vec.wrapping_add(0x20)), zero),
            ));
            let o1 = store(add(
                add(
                    mul(load(vec.wrapping_add(0x04)), c),
                    mul(load(vec.wrapping_add(0x14)), zero),
                ),
                mul(load(vec.wrapping_add(0x24)), zero),
            ));
            let o2 = store(add(
                add(
                    mul(load(vec.wrapping_add(0x08)), c),
                    mul(load(vec.wrapping_add(0x18)), zero),
                ),
                mul(load(vec.wrapping_add(0x28)), zero),
            ));
            let outs = [o0, o1, o2];
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
