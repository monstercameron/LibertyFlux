// original: 0x00C87F40 audio_mix_transform_gate_emit (proposed)

/// Mix three weighted sums, optionally transform them, and emit on threshold.
///
/// `this` carries the entry index (u16 at `+4`), the entry table (at
/// `+0xF30`, entries of 96 bytes) and a threshold float (at `+0x20`). `a2`
/// carries six base floats (`+0x90`..`+0xA8`) and a coefficient table (at
/// `+0xB0`) of signed 16-bit triples. `a4` is a word table; `a5`, `a6`, `a7`
/// index words in it (at `+0x10` past twice the index), each word selecting
/// one coefficient triple. `a1` is an object with a 12-float matrix
/// (`+0x00`..`+0x38`), a flag byte (`+0x40`), a chain pointer (`+0x44`) and a
/// word table (`+0x50`). `a3` points at two floats. `a0` is a word-table
/// index. `a8`/`a9` are flag words (low bytes gate the call path; full words
/// are passed on); `a10` is passed through untouched.
///
/// Three verses each form one triple: `t[i] = base[i] + weight * coef[i]`
/// with the verse's coefficient triple converted from i16. When the chain
/// (`[a1+0x44]` then `+0x38`, both non-null, the second not `-0x10`) is
/// present and the flag byte is zero, each triple is transformed by the
/// matrix (`r = m * t + m_last`, in the original's exact order; the second
/// triple's middle term legitimately lacks its matrix factor, as the code
/// shows). Otherwise the raw triples are used.
///
/// The final stage mixes the triples with the two floats at `a3`
/// (differences, a division-by-three average, halved sums) and compares
/// seven sum-of-squares values against the threshold with ordered `>`
/// (NaN never takes the call path). When any comparison is above and either
/// flag byte is non-zero, callee 1 runs with the entry pointer in `ecx` and
/// `(triple0, triple1, triple2, a10, 0xFF, a8, a9, chain)`, where each
/// triple argument points at the four corresponding frame words (three
/// floats plus a zero word). When it answers non-zero, the entry index is
/// stored into the word table at `a0` and, if the entry's flag byte has bit
/// 2, callee 2 runs on the global object with the entry, then bit 4 is set.
/// The function returns nothing.
///
/// Float evaluation decides operand order by a branch (see `fadd`): the
/// destination register decides the sign of a NaN result, and the compiler
/// swaps the operands of scalar operations even across call boundaries, so
/// both-NaN pairs forward the destination operand explicitly while every
/// other case runs as a plain operation whose order is irrelevant.
///
/// Original: 0x00C87F40 (thiscall with eleven stack words, callee pops
/// 0x2C; reads one uninitialized frame word whose value is the checker's
/// defined stack fill, and ends with the CRT security-cookie check, which
/// the rewrite performs as a preserving call).
lf_checker_rt::export!(thiscall, rw_00C87F40(
    this: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    a8: u32,
    a9: u32,
    a10: u32,
) -> u32 {
    unsafe {
        const ENTRY_OFF: u32 = 0xF30;
        const ENTRY_SUB: u32 = 0x60;
        const ENTRY_STRIDE: u32 = 32;
        const FLAG_OFF: u32 = 0x55;
        const FLAG_BIT: u8 = 2;
        const FLAG_SET: u8 = 4;
        const GLOBAL_OBJ: u32 = 0x1683290;
        const CALLEE_EMIT: u32 = 1;
        const CALLEE_TOUCH: u32 = 2;
        const CALLEE_COOKIE: u32 = 3;
        const THIRD: u32 = 0x3EAAAAAB;
        const HALF: u32 = 0x3F000000;
        const IDX_BASE: u32 = 0x10;

        type F = u32; // bits of one f32
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const u16).read_unaligned() as i16 as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        // Operand order is decided by a branch, not by the compiler: the
        // destination register decides a NaN result's sign, and the
        // compiler swaps the operands of scalar float operations even
        // across call boundaries (seen in the DLL: dest became the second
        // argument). Order matters only when both operands are NaN â€” with
        // one NaN the result is that NaN either way, and invalid cases
        // yield the same default NaN â€” so both-NaN pairs forward the
        // destination operand's bits (quiet bit set) exactly as the
        // hardware does, and everything else runs as a plain operation
        // whose operand order is then irrelevant. SNaN cannot occur
        // (audited: no SNaN in any input pool, and no operation here
        // creates one from non-SNaN inputs).
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
        #[inline(never)]
        fn fsub(a: F, b: F) -> F {
            unsafe {
                let va = core::arch::x86::_mm_set_ss(f32::from_bits(a));
                let vb = core::arch::x86::_mm_set_ss(f32::from_bits(b));
                let mut o = 0f32;
                core::arch::x86::_mm_store_ss(
                    &mut o,
                    core::arch::x86::_mm_sub_ss(va, vb),
                );
                o.to_bits()
            }
        }
        #[inline(always)]
        fn cvt(i: i32) -> F {
            unsafe {
                let v = core::arch::x86::_mm_cvtepi32_ps(
                    core::arch::x86::_mm_set_epi32(0, 0, 0, i),
                );
                let mut o = 0f32;
                core::arch::x86::_mm_store_ss(&mut o, v);
                o.to_bits()
            }
        }
        #[inline(always)]
        fn gt(a: F, b: F) -> bool {
            f32::from_bits(a) > f32::from_bits(b)
        }

        // Frame shadow for the three snapshot windows (four words each:
        // one output triple plus a zero word). The original's frame holds
        // the defined stack fill (zero here) in the fourth word of each
        // window, so a zeroed array matches in every path.
        let mut shadow = [0u32; 12];

        // Three verses: t[i] = base[i] + weight * coef_triple[i].
        let b90 = rd32(a2.wrapping_add(0x90));
        let b94 = rd32(a2.wrapping_add(0x94));
        let b98 = rd32(a2.wrapping_add(0x98));
        let bA0 = rd32(a2.wrapping_add(0xA0));
        let bA4 = rd32(a2.wrapping_add(0xA4));
        let bA8 = rd32(a2.wrapping_add(0xA8));
        let coef = rd32(a2.wrapping_add(0xB0));
        let idx5 = rd16(a4.wrapping_add(a5.wrapping_mul(2)).wrapping_add(IDX_BASE));
        let c5 = idx5.wrapping_mul(3).wrapping_mul(2);
        let k50 = cvt(rd16s(coef.wrapping_add(c5)));
        let k51 = cvt(rd16s(coef.wrapping_add(c5).wrapping_add(2)));
        let k52 = cvt(rd16s(coef.wrapping_add(c5).wrapping_add(4)));
        let t50 = fadd(bA0, fmul(b90, k50));
        let t51 = fadd(bA4, fmul(b94, k51));
        let t52 = fadd(bA8, fmul(b98, k52));
        let idx6 = rd16(a4.wrapping_add(a6.wrapping_mul(2)).wrapping_add(IDX_BASE));
        let c6 = idx6.wrapping_mul(3).wrapping_mul(2);
        let k60 = cvt(rd16s(coef.wrapping_add(c6)));
        let k61 = cvt(rd16s(coef.wrapping_add(c6).wrapping_add(2)));
        let k62 = cvt(rd16s(coef.wrapping_add(c6).wrapping_add(4)));
        let t60 = fadd(bA0, fmul(b90, k60));
        let t61 = fadd(bA4, fmul(b94, k61));
        let t62 = fadd(bA8, fmul(b98, k62));
        let idx7 = rd16(a4.wrapping_add(a7.wrapping_mul(2)).wrapping_add(IDX_BASE));
        let c7 = idx7.wrapping_mul(3).wrapping_mul(2);
        let k70 = cvt(rd16s(coef.wrapping_add(c7)));
        let k71 = cvt(rd16s(coef.wrapping_add(c7).wrapping_add(2)));
        let k72 = cvt(rd16s(coef.wrapping_add(c7).wrapping_add(4)));
        let t70 = fadd(bA0, fmul(b90, k70));
        let t71 = fadd(bA4, fmul(b94, k71));
        let t72 = fadd(bA8, fmul(b98, k72));

        // Conditional matrix transform.
        let ch0 = rd32(a1.wrapping_add(0x44));
        let ch1 = if ch0 != 0 { rd32(ch0.wrapping_add(0x38)) } else { 0 };
        let transform =
            ch0 != 0 && ch1 != 0 && ch1.wrapping_add(0x10) != 0 && rd8(a1.wrapping_add(0x40)) == 0;
        // Selected triples and convergent accumulators.
        let (v48, v04, v38, v64, v28, v24, c7v, c3v);
        if transform {
            let m00 = rd32(a1);
            let m04 = rd32(a1.wrapping_add(0x04));
            let m08 = rd32(a1.wrapping_add(0x08));
            let m10 = rd32(a1.wrapping_add(0x10));
            let m14 = rd32(a1.wrapping_add(0x14));
            let m18 = rd32(a1.wrapping_add(0x18));
            let m20 = rd32(a1.wrapping_add(0x20));
            let m24 = rd32(a1.wrapping_add(0x24));
            let m28 = rd32(a1.wrapping_add(0x28));
            let m30 = rd32(a1.wrapping_add(0x30));
            let m34 = rd32(a1.wrapping_add(0x34));
            let m38 = rd32(a1.wrapping_add(0x38));
            let r10 = fadd(
                fadd(fadd(fmul(m10, t51), fmul(m00, t50)), fmul(m20, t52)),
                m30,
            );
            let r11 = fadd(
                fadd(fadd(fmul(m04, t50), fmul(m14, t51)), fmul(m24, t52)),
                m34,
            );
            let r12 = fadd(
                fadd(fadd(fmul(t51, m18), fmul(t50, m08)), fmul(t52, m28)),
                m38,
            );
            let r20 = fadd(
                fadd(fadd(fmul(m00, t60), fmul(m10, t61)), fmul(m20, t62)),
                m30,
            );
            let r21 = fadd(
                fadd(fadd(fmul(m04, t60), fmul(m14, t61)), fmul(m24, t62)),
                m34,
            );
            // Note the missing matrix factor on the middle term: the code
            // adds the raw second value, not multiplied by m18.
            let r22 = fadd(fadd(fadd(fmul(t60, m08), t61), fmul(t62, m28)), m38);
            let r30 = fadd(
                fadd(fadd(fmul(m00, t70), fmul(m10, t71)), fmul(m20, t72)),
                m30,
            );
            let r31 = fadd(
                fadd(fadd(fmul(m04, t70), fmul(m14, t71)), fmul(m24, t72)),
                m34,
            );
            let r32 = fadd(
                fadd(fadd(fmul(t70, m08), fmul(t71, m18)), fmul(t72, m28)),
                m38,
            );
            shadow[0] = r10;
            shadow[1] = r11;
            shadow[2] = r12;
            shadow[4] = r20;
            shadow[5] = r21;
            shadow[6] = r22;
            shadow[8] = r30;
            shadow[9] = r31;
            shadow[10] = r32;
            v48 = r10;
            v04 = r11;
            v38 = r20;
            v64 = r21;
            v28 = r30;
            v24 = r31;
            c7v = r30;
            c3v = r31;
        } else {
            shadow[0] = t50;
            shadow[1] = t51;
            shadow[2] = t52;
            shadow[4] = t60;
            shadow[5] = t61;
            shadow[6] = t62;
            shadow[8] = t70;
            shadow[9] = t71;
            shadow[10] = t72;
            v48 = t50;
            v04 = t51;
            v38 = t60;
            v64 = t61;
            v28 = t70;
            v24 = t71;
            c7v = t70;
            c3v = t71;
        }

        // Final mix with the two floats at a3.
        let s0 = fadd(v38, v48);
        let s1 = fadd(v64, v04);
        let a3v0 = rd32(a3);
        let a3v1 = rd32(a3.wrapping_add(4));
        let x2 = fmul(fadd(c7v, s0), THIRD);
        let d0 = fsub(a3v0, v38);
        let x3 = fmul(fadd(c3v, s1), THIRD);
        let d1 = fsub(a3v1, v04);
        let d2 = fsub(a3v1, v64);
        let d3 = fsub(a3v0, v28);
        let e0 = fsub(a3v0, x2);
        let e1 = fsub(a3v0, v48);
        let d4 = fsub(a3v1, v24);
        let e2 = fsub(a3v1, x3);
        let f0 = fmul(fadd(d2, d1), HALF);
        let f1 = fmul(fadd(d0, e1), HALF);
        let f2 = fmul(fadd(d3, d0), HALF);
        let f3 = fmul(fadd(d4, d2), HALF);
        let f4 = fmul(fadd(d4, d1), HALF);
        let f5 = fmul(fadd(d3, e1), HALF);
        let thr = rd32(this.wrapping_add(0x20));
        let g0 = fadd(fmul(e1, e1), fmul(d1, d1));
        let take = gt(thr, g0)
            || gt(thr, fadd(fmul(d0, d0), fmul(d2, d2)))
            || gt(thr, fadd(fmul(d3, d3), fmul(d4, d4)))
            || gt(thr, fadd(fmul(e0, e0), fmul(e2, e2)))
            || gt(thr, fadd(fmul(f1, f1), fmul(f0, f0)))
            || gt(thr, fadd(fmul(f2, f2), fmul(f3, f3)))
            || gt(thr, fadd(fmul(f5, f5), fmul(f4, f4)));

        // Call path.
        if take && ((a8 & 0xFF) != 0 || (a9 & 0xFF) != 0) {
            let w = rd16(this.wrapping_add(4));
            let base = rd32(this.wrapping_add(ENTRY_OFF));
            let entry = base
                .wrapping_add(w.wrapping_mul(3).wrapping_mul(ENTRY_STRIDE))
                .wrapping_sub(ENTRY_SUB);
            let chain = rd32(a1.wrapping_add(0x44));
            let ans = lf_checker_rt::callee_thiscall!(
                CALLEE_EMIT,
                u32,
                entry,
                shadow.as_ptr() as u32,
                unsafe { shadow.as_ptr().add(4) as u32 },
                unsafe { shadow.as_ptr().add(8) as u32 },
                a10,
                0xFF,
                a8,
                a9,
                chain
            );
            if ans != 0 {
                let wtab = rd32(a1.wrapping_add(0x50));
                wr16(wtab.wrapping_add(a0.wrapping_mul(2)), w as u16);
                if rd8(entry.wrapping_add(FLAG_OFF)) & FLAG_BIT != 0 {
                    lf_checker_rt::callee_thiscall!(
                        CALLEE_TOUCH,
                        u32,
                        lf_checker_rt::relocated(GLOBAL_OBJ),
                        entry
                    );
                    wr8(
                        entry.wrapping_add(FLAG_OFF),
                        rd8(entry.wrapping_add(FLAG_OFF)) | FLAG_SET,
                    );
                }
            }
        }
        lf_checker_rt::callee_stdcall!(CALLEE_COOKIE, u32,);
        0
    }
});
