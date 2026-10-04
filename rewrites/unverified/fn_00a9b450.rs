// original: 0x00A9B450 files-memory spatial resample (unnamed in symbols)
/// Stage-3 rewrite: entry frame, transform call, float block, sphere
/// loop over the entry list, tail block (store and blend-call variants),
/// counter checks and the full epilogue. Later stages add the primer
/// loop and the scatter loop; see the lane report.
lf_checker_rt::export!(thiscall, rb110_fn3s3(this: u32, arg0: u32, arg1: u32) -> u32 {
    // Intercepted callees (ids match the contract's `callees` table).
    const CAL_XFORM: u32 = 1; // matrix transform (thiscall/1, frame blocks)
    const CAL_COOKIE: u32 = 2; // frame-cookie check (cdecl/0)
    const CAL_COUNT: u32 = 3; // entry counter (thiscall/0)
    const CAL_BLEND: u32 = 4; // vector blend (cdecl/6, frame out-params)

    /// Scalar float add with the original's exact NaN propagation: a NaN
    /// destination wins (quieted), else a NaN source wins (quieted);
    /// otherwise a plain add, which is order-free without NaNs. Never
    /// inlined, so the operation order of a chain cannot be reassociated.
    #[inline(never)]
    fn fadd_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest + src
    }

    /// Scalar float multiply with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fmul_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest * src
    }

    /// Scalar float subtract with the same NaN rule as fadd_ss above.
    #[inline(never)]
    fn fsub_ss(dest: f32, src: f32) -> f32 {
        let a = dest.to_bits();
        if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
            return f32::from_bits(a | 0x00400000);
        }
        let b = src.to_bits();
        if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
            return f32::from_bits(b | 0x00400000);
        }
        dest - src
    }

    unsafe {
        // Entry: owner -> matrix object, 12 matrix floats (the source
        // has no fourth column; frame slots without a source read the
        // defined stack fill, 0.0, exactly as the original does).
        let o68 = *((arg0.wrapping_add(0x68)) as *const u32);
        let mat = *((o68.wrapping_add(0x20)) as *const u32);
        let m00 = *((mat.wrapping_add(0x00)) as *const f32);
        let m04 = *((mat.wrapping_add(0x04)) as *const f32);
        let m08 = *((mat.wrapping_add(0x08)) as *const f32);
        let m10 = *((mat.wrapping_add(0x10)) as *const f32);
        let m14 = *((mat.wrapping_add(0x14)) as *const f32);
        let m18 = *((mat.wrapping_add(0x18)) as *const f32);
        let m20 = *((mat.wrapping_add(0x20)) as *const f32);
        let m24 = *((mat.wrapping_add(0x24)) as *const f32);
        let m28 = *((mat.wrapping_add(0x28)) as *const f32);
        let m30 = *((mat.wrapping_add(0x30)) as *const f32);
        let m34 = *((mat.wrapping_add(0x34)) as *const f32);
        let m38 = *((mat.wrapping_add(0x38)) as *const f32);
        // Both blocks start as the same 4x4 matrix with an empty fourth
        // column. Note: the stores after the first push address one word
        // lower than their displacement says.
        let src = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        let mut dst = [
            m00, m04, m08, 0.0, m10, m14, m18, 0.0, m20, m24, m28, 0.0, m30,
            m34, m38, 0.0,
        ];
        // Transform: the callee fills the block's first row.
        lf_checker_rt::callee_thiscall!(
            CAL_XFORM,
            u32,
            dst.as_mut_ptr() as u32,
            src.as_ptr() as u32
        );
        // Context vector, then the three dot products below.
        let e10 = *((arg1.wrapping_add(0x10)) as *const f32);
        let e14 = *((arg1.wrapping_add(0x14)) as *const f32);
        let e18 = *((arg1.wrapping_add(0x18)) as *const f32);
        // Float block: three dot products over the block and the
        // context vector, one scalar operation per statement in the
        // original's exact order (see fadd_ss).
        let a40 = fmul_ss(dst[4], e14);
        let b40 = fmul_ss(dst[0], e10);
        let s40 = fadd_ss(a40, b40);
        let c40 = fmul_ss(dst[8], e18);
        let t40 = fadd_ss(s40, c40);
        let f40 = fadd_ss(t40, dst[12]);
        let a2c = fmul_ss(dst[5], e14);
        let b2c = fmul_ss(dst[1], e10);
        let s2c = fadd_ss(a2c, b2c);
        let c2c = fmul_ss(dst[9], e18);
        let t2c = fadd_ss(s2c, c2c);
        let f2c = fadd_ss(t2c, dst[13]);
        let a20 = fmul_ss(dst[6], e14);
        let b20 = fmul_ss(dst[2], e10);
        let s20 = fadd_ss(a20, b20);
        let c20 = fmul_ss(dst[10], e18);
        let t20 = fadd_ss(s20, c20);
        let f20 = fadd_ss(t20, dst[14]);
        // Sphere loop over the entry list: each flagged node carries
        // three spheres, and a radius covering the point exits early.
        let mut use_hit_path = false;
        let mut cur = *((this.wrapping_add(0x0C)) as *const u32);
        while cur != 0 {
            let flagged = *((cur.wrapping_add(0xF0)) as *const u8) != 0;
            let next = *(cur as *const u32);
            if flagged {
                let r = *((arg1.wrapping_add(0x20)) as *const f32);
                let r2 = fmul_ss(r, r);
                let mut k = 0u32;
                while k < 3 {
                    let base = cur.wrapping_add(0x10).wrapping_add(k.wrapping_mul(0x10));
                    let dx = fsub_ss(*(base as *const f32), f40);
                    let dy = fsub_ss(
                        *((base.wrapping_add(4)) as *const f32),
                        f2c,
                    );
                    let dz = fsub_ss(
                        *((base.wrapping_add(8)) as *const f32),
                        f20,
                    );
                    let dx2 = fmul_ss(dx, dx);
                    let dy2 = fmul_ss(dy, dy);
                    let dz2 = fmul_ss(dz, dz);
                    let d2 = fadd_ss(fadd_ss(dx2, dy2), dz2);
                    // Ordered greater-or-equal, matching comiss+jae.
                    if r2 >= d2 {
                        use_hit_path = true;
                        break;
                    }
                    k = k.wrapping_add(1);
                }
                if use_hit_path {
                    break;
                }
            }
            cur = next;
        }
        if !use_hit_path {
            let flag85 = *((arg0.wrapping_add(0x85)) as *const u8);
            if flag85 == 0 {
                // Early epilogue.
                lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
                return 0;
            }
        }
        // Tail block: the list primer is skipped when already set
        // (stages 1-3 pin it set on tail rows).
        if *((this.wrapping_add(0x14)) as *const u8) != 0 {
            let c0 = *((this.wrapping_add(0xC0)) as *const f32);
            if c0 == 0.0 {
                // Store variant: publish the three products.
                *((this.wrapping_add(0xB0)) as *mut f32) = f40;
                *((this.wrapping_add(0xB4)) as *mut f32) = f2c;
                *((this.wrapping_add(0xB8)) as *mut f32) = f20;
                *((this.wrapping_add(0xBC)) as *mut f32) = 0.0;
                let e20 = *((arg1.wrapping_add(0x20)) as *const u32);
                *((this.wrapping_add(0xC0)) as *mut u32) = e20;
            } else {
                // Blend-call variant: the callee fills the outputs
                // through frame pointers; its float arguments are the
                // current marker and context words (the original
                // overwrites two pushed slots with them).
                let mut out = [0.0f32; 4];
                let mut mark = 0.0f32;
                let c0bits = *((this.wrapping_add(0xC0)) as *const u32);
                let e20bits = *((arg1.wrapping_add(0x20)) as *const u32);
                lf_checker_rt::callee_cdecl!(
                    CAL_BLEND,
                    u32,
                    this.wrapping_add(0xB0),
                    c0bits,
                    &f40 as *const f32 as u32,
                    e20bits,
                    out.as_mut_ptr() as u32,
                    &mut mark as *mut f32 as u32,
                );
                *((this.wrapping_add(0xB0)) as *mut f32) = out[0];
                *((this.wrapping_add(0xB4)) as *mut f32) = out[1];
                *((this.wrapping_add(0xB8)) as *mut f32) = out[2];
                *((this.wrapping_add(0xBC)) as *mut f32) = out[3];
                *((this.wrapping_add(0xC0)) as *mut f32) = mark;
            }
            // Counter checks: an over-limit slot count or entry
            // count raises the sticky flag (unsigned, then signed).
            if *((this.wrapping_add(8)) as *const u32) > 0x80 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            let n = lf_checker_rt::callee_thiscall!(CAL_COUNT, u32, arg0);
            if (n as i32) > 0x200 {
                *((this.wrapping_add(0x15)) as *mut u8) = 1;
            }
            // Second list loop: skeleton (flag skips only; per-node
            // bodies are stage 5). The processed count gates the
            // scatter tail, which stage 3 never reaches.
            let mut processed = 0u32;
            let mut cur2 = *((this.wrapping_add(0x0C)) as *const u32);
            while cur2 != 0 {
                let live = *((cur2.wrapping_add(0xF0)) as *const u8) != 0;
                let next2 = *(cur2 as *const u32);
                let done = *((cur2.wrapping_add(0xF4)) as *const u32) != 0;
                if live && !done {
                    // Stage-5 body (unreached: stage 3 pins every
                    // sphere node done).
                    processed = processed.wrapping_add(1);
                }
                cur2 = next2;
            }
            if processed > 0 {
                // Scatter tail (stage 5).
            }
            // Full epilogue: copy the context tag, then check.
            let tag = *((arg1.wrapping_add(0x28)) as *const u8);
            *((arg0.wrapping_add(0x84)) as *mut u8) = tag;
            lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
            return 0;
        }
        // Beyond stage 3 (unreached by the stage-3 contract).
        lf_checker_rt::callee_cdecl!(CAL_COOKIE, u32,);
        0
    }
});
