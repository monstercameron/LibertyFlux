// original: 0x00634270 motion_sample_resolve (proposed)

/// Resolve one motion sample: tokenize its tags, read its channels, and
/// build its rotation matrix.
///
/// `arg1` handles a reader (sized-fill at vtable slot 2, float channel at
/// slot 0x18, cursor at `+0x10`); `this` collects a parse slot plus a
/// channel block (`+0xC0` nonzero means initialised, `+0xCA`/`+0xC8` tags,
/// `+0xC4` dword table, `+0xCC` 64-byte-stride pool, `+0xD0` 16-bit used
/// count). The header match passes (buffer, tag) but every later stage
/// passes (tag, buffer) to the same matcher; the order differs on purpose.
/// When initialised the block grows by one row (full allocation
/// only if the tag word is still zero, through a size helper and a
/// thread-local pool allocator); the destination is the pool row, else the
/// inline block at `this+0x80`. A "none"-tagged header optionally parses
/// one integer into the row slot. Then single-shot "<"/">" match stages
/// read three channel floats (or zero them when the first stage misses)
/// and a four-float quaternion; the quaternion's squared length selects a
/// normalising reciprocal-square-root path (skipped exactly when the
/// length compares equal to +0/-0, NaN included on the run side) and the
/// scaled quaternion expands into a 3x3 matrix with 1.0 and sqrt(2)
/// constants. A missed quaternion tag bails out with an identity matrix
/// instead. Returns the reader cursor saved on entry to the final stage
/// (matrix and bail-out paths alike), or 0 when the final tag matches.
/// Every comparison on a callee answer is null/zero-equality;
/// nothing here compares signed versus unsigned. thiscall, one stack word.
/// Floats replicate the original SSE op order exactly (operands pinned);
/// the rewrite keeps quaternions in locals rather than stack slots.
lf_checker_rt::export!(thiscall, rw_00634270(this: u32, arg1: u32) -> u32 {
    unsafe {
        const VT_FILL: u32 = 0x08;
        const VT_CHAN: u32 = 0x18;
        const RDR_CURSOR: u32 = 0x10;
        const TAG_NONE: u32 = 0x00F9_7DF0;
        const TAG_LT_A: u32 = 0x00F9_7DC8;
        const TAG_LT_B: u32 = 0x00F9_7DCC;
        const TAG_GT_A: u32 = 0x00F9_7DD0;
        const TAG_LT_C: u32 = 0x00F9_7DD4;
        const TAG_LT_D: u32 = 0x00F9_7E0C;
        const TAG_GT_B: u32 = 0x00F9_7E10;
        const ONE_BITS: u32 = 0x3F80_0000;
        const SQRT2_BITS: u32 = 0x3FB5_04F3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn vfill(obj: u32, buf: *mut u8, n: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_FILL));
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, buf as u32, n)
            }
        }
        #[inline(always)]
        unsafe fn vchan(obj: u32) -> f32 {
            unsafe {
                let vt: u32 = rd32(obj);
                let slot: u32 = rd32(vt.wrapping_add(VT_CHAN));
                let f: extern "thiscall" fn(u32, u32) -> f32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(obj, 1)
            }
        }
        #[inline(always)]
        unsafe fn valloc(alloc: u32, n: u32) -> u32 {
            unsafe {
                let vt: u32 = rd32(alloc);
                let slot: u32 = rd32(vt.wrapping_add(VT_FILL));
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                f(alloc, n, 0x10, 0)
            }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fdiv(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let mut buf_x = [0u8; 64];
        let mut buf_y = [0u8; 64];
        let mut dest: u32 = this.wrapping_add(0x80);
        let mut parse_target: u32 = this.wrapping_add(0xC0);
        if rd32(this.wrapping_add(0xC0)) != 0 {
            if (this.wrapping_add(0xCA) as *const u16).read_unaligned() == 0 {
                (this.wrapping_add(0xCA) as *mut u16).write_unaligned(0x40);
                let tbl: u32 = lf_checker_rt::callee_thiscall!(
                    3, u32, this.wrapping_add(0xC4), 0x40
                );
                wr32(this.wrapping_add(0xC4), tbl);
                (this.wrapping_add(0xC8) as *mut u16).write_unaligned(0x40);
                let tls0: u32 = lf_checker_rt::tls_slot(0);
                let alloc: u32 = rd32(tls0.wrapping_add(8));
                wr32(this.wrapping_add(0xCC), valloc(alloc, 0x1000));
            }
            let count: u32 =
                (this.wrapping_add(0xD0) as *const u16).read_unaligned() as u32;
            parse_target =
                rd32(this.wrapping_add(0xC4)).wrapping_add(count.wrapping_mul(4));
            dest = rd32(this.wrapping_add(0xCC))
                .wrapping_add(count.wrapping_mul(64));
            (this.wrapping_add(0xD0) as *mut u16)
                .write_unaligned(count.wrapping_add(1) as u16);
        }
        let obj: u32 = rd32(arg1.wrapping_add(4));
        // Header: "none" tag, optional integer parse into the row slot.
        buf_x[0] = 0;
        vfill(obj, buf_x.as_mut_ptr(), 0x200);
        let t0: u32 = lf_checker_rt::callee_cdecl!(
            5, u32, buf_x.as_mut_ptr() as u32, lf_checker_rt::relocated(TAG_NONE)
        );
        if t0 != 0 {
            let v: u32 = lf_checker_rt::callee_thiscall!(
                6, u32, buf_x.as_mut_ptr() as u32
            );
            wr32(parse_target, v);
        }
        // Stage 1a: "<" tag; miss zeroes the channel triplet.
        let mut save12: u32 = rd32(obj.wrapping_add(RDR_CURSOR));
        buf_y[0] = 0;
        let mut slot08: u32 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
        let t1: u32 = if slot08 == 0 {
            1
        } else {
            lf_checker_rt::callee_cdecl!(
                5, u32, lf_checker_rt::relocated(TAG_LT_A), buf_y.as_mut_ptr() as u32
            )
        };
        if slot08 == 0 || t1 != 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
            );
            wr32(obj.wrapping_add(RDR_CURSOR), save12);
            wr32(dest.wrapping_add(0x30), 0);
            wr32(dest.wrapping_add(0x34), 0);
            wr32(dest.wrapping_add(0x38), 0);
        } else {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
            );
            wr32(obj.wrapping_add(RDR_CURSOR), save12);
            // Stage 1b: second "<" tag; either way the channels are read.
            buf_y[0] = 0;
            slot08 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
            if slot08 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
                );
                wr32(obj.wrapping_add(RDR_CURSOR), save12);
            } else {
                let t: u32 = lf_checker_rt::callee_cdecl!(
                    5, u32, lf_checker_rt::relocated(TAG_LT_B), buf_y.as_mut_ptr() as u32
                );
                if t != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
                    );
                    wr32(obj.wrapping_add(RDR_CURSOR), save12);
                }
            }
            (dest.wrapping_add(0x30) as *mut f32).write_unaligned(vchan(obj));
            (dest.wrapping_add(0x34) as *mut f32).write_unaligned(vchan(obj));
            (dest.wrapping_add(0x38) as *mut f32).write_unaligned(vchan(obj));
            // Stage 2: ">" tag; a match steps past nothing.
            save12 = rd32(obj.wrapping_add(RDR_CURSOR));
            buf_y[0] = 0;
            slot08 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
            if slot08 == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
                );
                wr32(obj.wrapping_add(RDR_CURSOR), save12);
            } else {
                let t: u32 = lf_checker_rt::callee_cdecl!(
                    5, u32, lf_checker_rt::relocated(TAG_GT_A), buf_y.as_mut_ptr() as u32
                );
                if t != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
                    );
                    wr32(obj.wrapping_add(RDR_CURSOR), save12);
                }
            }
        }
        // Stage 3: "<" tag; a miss bails out with identity.
        save12 = rd32(obj.wrapping_add(RDR_CURSOR));
        buf_y[0] = 0;
        slot08 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
        let mut bail: bool = slot08 == 0;
        if !bail {
            let t: u32 = lf_checker_rt::callee_cdecl!(
                5, u32, lf_checker_rt::relocated(TAG_LT_C), buf_y.as_mut_ptr() as u32
            );
            bail = t != 0;
        }
        if bail {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
            );
            wr32(obj.wrapping_add(RDR_CURSOR), save12);
            wr32(dest, ONE_BITS);
            wr32(dest.wrapping_add(4), 0);
            wr32(dest.wrapping_add(8), 0);
            wr32(dest.wrapping_add(0x10), 0);
            wr32(dest.wrapping_add(0x14), ONE_BITS);
            wr32(dest.wrapping_add(0x18), 0);
            wr32(dest.wrapping_add(0x20), 0);
            wr32(dest.wrapping_add(0x24), 0);
            wr32(dest.wrapping_add(0x28), ONE_BITS);
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, 0);
            return save12;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(
            7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
        );
        wr32(obj.wrapping_add(RDR_CURSOR), save12);
        // Stage 4: "<" tag; either way the quaternion is read.
        save12 = rd32(obj.wrapping_add(RDR_CURSOR));
        buf_y[0] = 0;
        slot08 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
        if slot08 == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
            );
            wr32(obj.wrapping_add(RDR_CURSOR), save12);
        } else {
            let t: u32 = lf_checker_rt::callee_cdecl!(
                5, u32, lf_checker_rt::relocated(TAG_LT_D), buf_y.as_mut_ptr() as u32
            );
            if t != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, obj, buf_y.as_mut_ptr() as u32, slot08
                );
                wr32(obj.wrapping_add(RDR_CURSOR), save12);
            }
        }
        let q0: f32 = vchan(obj);
        let q1: f32 = vchan(obj);
        let q2: f32 = vchan(obj);
        let q3: f32 = vchan(obj);
        // Squared length in the original add order, then the gate.
        let mut x1: f32 = fmul(q1, q1);
        x1 = fadd(x1, fmul(q0, q0));
        x1 = fadd(x1, fmul(q2, q2));
        x1 = fadd(x1, fmul(q3, q3));
        let mut x6: f32 = 0.0;
        let mut x2: f32 = q0;
        if x1 != 0.0 {
            let s: f32 = lf_checker_rt::callee_cdecl!(8, f32, x1.to_bits());
            x2 = q0;
            x6 = fdiv(1.0, s);
            // The original spills the root over the stage-4 fill answer.
            slot08 = s.to_bits();
        }
        let x0c: f32 = f32::from_bits(SQRT2_BITS);
        let mut x4: f32 = fmul(fmul(x6, q1), x0c);
        let mut x5: f32 = fmul(fmul(x6, x2), x0c);
        let mut x3: f32 = x6;
        x6 = fmul(fmul(x6, q3), x0c);
        x3 = fmul(fmul(x3, q2), x0c);
        x1 = fmul(x4, x5);
        x2 = fmul(x6, x3);
        {
            let x0: f32 = fadd(x1, x2);
            let m01: f32 = x0;
            x1 = fsub(x1, x2);
            let m10: f32 = x1;
            x2 = fmul(x3, x5);
            (dest.wrapping_add(4) as *mut f32).write_unaligned(m01);
            (dest.wrapping_add(0x10) as *mut f32).write_unaligned(m10);
            x1 = fmul(x6, x4);
            x6 = fmul(x6, x5);
            x5 = fmul(x5, x5);
            let x0b: f32 = fsub(x2, x1);
            x1 = fadd(x1, x2);
            (dest.wrapping_add(8) as *mut f32).write_unaligned(x0b);
            (dest.wrapping_add(0x20) as *mut f32).write_unaligned(x1);
            x1 = fmul(x3, x4);
            x3 = fmul(x3, x3);
            let x0c2: f32 = fadd(x6, x1);
            x1 = fsub(x1, x6);
            x4 = fmul(x4, x4);
            (dest.wrapping_add(0x18) as *mut f32).write_unaligned(x0c2);
            (dest.wrapping_add(0x24) as *mut f32).write_unaligned(x1);
            x1 = fadd(x3, x4);
            x3 = fadd(x3, x5);
            x4 = fadd(x4, x5);
            let one: f32 = 1.0;
            let m00: f32 = fsub(one, x1);
            (dest.wrapping_add(0) as *mut f32).write_unaligned(m00);
            let m11: f32 = fsub(one, x3);
            let m22: f32 = fsub(one, x4);
            (dest.wrapping_add(0x14) as *mut f32).write_unaligned(m11);
            (dest.wrapping_add(0x28) as *mut f32).write_unaligned(m22);
        }
        // Stage 5: ">" tag; a match skips the step. The restore uses the
        // stage-entry cursor: while pushing the fill arguments the original
        // spills the cursor over the stale slot (which held the square-root
        // result when it ran, else stage 4's fill answer), and the epilogue
        // returns that spilled cursor. A match skips the restore and the
        // step: eax still holds the strcmp's 0.
        buf_y[0] = 0;
        let e5: u32 = vfill(obj, buf_y.as_mut_ptr(), 0x200);
        if e5 == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, obj, buf_y.as_mut_ptr() as u32, e5
            );
            wr32(obj.wrapping_add(RDR_CURSOR), save12);
        } else {
            let t: u32 = lf_checker_rt::callee_cdecl!(
                5, u32, lf_checker_rt::relocated(TAG_GT_B), buf_y.as_mut_ptr() as u32
            );
            if t != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    7, u32, obj, buf_y.as_mut_ptr() as u32, e5
                );
                wr32(obj.wrapping_add(RDR_CURSOR), save12);
                let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, 0);
                return save12;
            }
            // A match skips the step: eax still holds the strcmp's 0.
            let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, 0);
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(9, u32, 0);
        save12
    }
});
