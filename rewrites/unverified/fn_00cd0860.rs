// original: 0x00CD0860 CTaskComplexMelee::vf20 (symbols)

/// Advance the complex melee task for one tick (virtual slot 20).
///
/// `this` is the melee task object, `arg` the ped it acts on. The task
/// object carries a linked object (`+LINK`), an id word (`+STATE`), flag
/// bytes (`+FLAGS`, `+FLAGS2`), a counter word (`+COUNT`), and scratch
/// fields; the ped carries flag bytes, a vtable-ish link and a large state
/// block. Returns a handler result, or 0 when the tick ends quietly
/// (thiscall, one stack word).
///
/// Behaviour: after stamping the ped and priming several subsystems (one of
/// which fills an eight-word scratch block), the ped flags select either an
/// early exit through the event dispatcher (when a virtual query answers
/// `0x11D`) or the main switch on a status word: 3 runs a gated hook and
/// returns 0; 1 resolves an object whose own query must differ from `0x38B`,
/// then runs the hook, a ten-argument setup call and the `0x38B` event,
/// returning its answer; 2 resolves a second object whose query must differ
/// from `0x3AE`, runs a three-argument check and the `0x3AE` event,
/// returning its answer. Any other status, or a rejected check, reaches the
/// tail: a flag-driven chain of small queries, a range-check call over the
/// scratch block, counter reset, more factory and setup calls, an event
/// buffer built over the scratch block and fired through the same gated
/// hook, an optional countdown computation (integer range scaled by a float
/// factor, truncated toward zero exactly like the original instruction),
/// two more setup calls, a flag-bit update and teardown, returning the
/// creation answer. Two out-pointers the original derives from uninitialised
/// stack slots are defined by the contract's zero fill.
///
/// Original: 0x00CD0860 (thiscall, one stack word; six indirect calls).
lf_checker_rt::export!(thiscall, rw_00CD0860(this: u32, arg: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x08;
        const STATE: u32 = 0x50;
        const FLAGS: u32 = 0xf0;
        const FLAGS2: u32 = 0xf1;
        const COUNT: u32 = 0xcc;
        const CREATOR: u32 = 0x167e2a0;
        const FACTORY_OBJ: u32 = 0x171c968;
        const EVT_TAG: u32 = 0xeb3acc;
        const G_COUNT_HI: u32 = 0x10519b8;
        const G_COUNT_LO: u32 = 0x10519b4;
        const G_STAMP: u32 = 0x11735b4;
        const K_STEP: u32 = 0xfe8680;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncate toward zero exactly like cvttss2si: NaN and
        /// out-of-range inputs give `i32::MIN`, unlike `as` saturation.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                i32::MIN
            } else {
                x as i32
            }
        }

        wr32(arg + 0x2a0, rd32(arg + 0x2a0) | 0x100);
        lf_checker_rt::callee_thiscall!(1, u32, this, arg);
        let st = rd32(this + STATE);
        if st != 0 && rd32(st + 0x28) & 0x3c0 == 0xc0 && rd8(st + 0x219) != 0 {
            wr32(arg + 0x29c, rd32(arg + 0x29c) | 0x80000000);
        }
        lf_checker_rt::callee_thiscall!(2, u32, this, arg);
        let mut buf = [0u32; 8];
        lf_checker_rt::callee_thiscall!(3, u32, this, arg, buf.as_mut_ptr() as u32);
        lf_checker_rt::callee_thiscall!(4, u32, this, arg);
        if !(rd8(arg + 0x218) == 0 && rd8(arg + 0x219) != 0) {
            lf_checker_rt::callee_thiscall!(5, u32, this, arg);
            let t8 = rd32(this + LINK);
            let q: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(t8) + 0x0c) as usize) };
            if q(t8) == 0x11d {
                return lf_checker_rt::callee_thiscall!(6, u32, this, arg);
            }
        }
        let t8 = rd32(this + LINK);
        let fstat: u32 = lf_checker_rt::callee_cdecl!(
            7, u32, t8, 0u32, lf_checker_rt::relocated(0x1112778),
            lf_checker_rt::relocated(0x10519c0), 0u32
        );
        let mut fc = fstat;
        let g8: u32 = lf_checker_rt::callee_thiscall!(8, u32, this);
        let h9: u32 =
            lf_checker_rt::callee_thiscall!(9, u32, this, g8 & 0xff);
        // The original passes an uninitialised stack slot as `this` here;
        // the contract fills it with zero on both sides.
        lf_checker_rt::callee_thiscall!(10, u32, 0u32, h9);
        let sw: u32 = lf_checker_rt::callee_thiscall!(11, u32, this, arg);
        let mut tail = false;
        if sw == 3 {
            if rd8(t8 + 0x0c) & 1 == 0 {
                let hk: extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
                    core::mem::transmute(rd32(rd32(t8) + 0x14) as usize)
                };
                if (hk(t8, arg, 1u32, 0u32) as u8) == 0 {
                    tail = true;
                } else {
                    wr32(t8 + 0x0c, rd32(t8 + 0x0c) | 2);
                }
            }
            if !tail {
                lf_checker_rt::callee_thiscall!(12, u32, this, arg);
                return 0;
            }
        } else if sw == 1 {
            let l1: u32 = lf_checker_rt::callee_thiscall!(
                13, u32, rd32(arg + 0x224)
            );
            let q1: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(l1) + 0x0c) as usize) };
            if q1(l1) == 0x38b {
                tail = true;
            } else {
                if rd8(t8 + 0x0c) & 1 == 0 {
                    let hk: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        unsafe {
                            core::mem::transmute(rd32(rd32(t8) + 0x14) as usize)
                        };
                    if (hk(t8, arg, 1u32, 0u32) as u8) == 0 {
                        tail = true;
                    } else {
                        wr32(t8 + 0x0c, rd32(t8 + 0x0c) | 2);
                    }
                }
                if !tail {
                    wr8(this + FLAGS2, rd8(this + FLAGS2) & 0xfb);
                    lf_checker_rt::callee_thiscall!(
                        14, u32, arg + 0x570, lf_checker_rt::relocated(0xeda160),
                        0u32, 0u32, 0u32, 0xffffffffu32, 0u32, 0u32,
                        0x3f800000u32, 0u32, 0u32
                    );
                    return lf_checker_rt::callee_thiscall!(
                        15, u32, this, arg, 0x38bu32, 0u32
                    );
                }
            }
        } else if sw == 2 {
            let l2: u32 = lf_checker_rt::callee_thiscall!(
                16, u32, rd32(arg + 0x224)
            );
            let q2: extern "thiscall" fn(u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(l2) + 0x0c) as usize) };
            if q2(l2) == 0x3ae {
                tail = true;
            } else {
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    17, u32, t8, arg, 1u32, 0u32
                );
                if (ok as u8) == 0 {
                    tail = true;
                } else {
                    return lf_checker_rt::callee_thiscall!(
                        18, u32, this, arg, 0x3aeu32, 0u32
                    );
                }
            }
        } else {
            tail = true;
        }
        if !tail {
            return 0;
        }
        // Tail: flag-driven query chain over the scratch block.
        let mut f10: u32;
        if fc != 0 {
            let o: u32 = lf_checker_rt::callee_thiscall!(19, u32, fc);
            buf[1] = (buf[1] & !0xff) | (o & 0xff);
            let p: u32 = lf_checker_rt::callee_thiscall!(20, u32, fc);
            if (p as u8) != 0 {
                let q: u32 = lf_checker_rt::callee_thiscall!(21, u32, fc);
                f10 = u32::from((q as u8) != 0);
            } else {
                f10 = 0;
            }
        } else {
            buf[1] = (buf[1] & !0xff) | 1;
            f10 = 0;
        }
        lf_checker_rt::callee_thiscall!(22, u32, this, arg, buf[1], f10, buf[0]);
        buf[0] = 0;
        buf[1] = 0;
        let r23: u32 = lf_checker_rt::callee_thiscall!(
            23, u32, this, arg, f10, buf.as_ptr() as u32 + 4, buf.as_ptr() as u32
        );
        let mut fb = (r23 as u8) & 0xff;
        if rd32(this + COUNT) != 0 {
            lf_checker_rt::callee_thiscall!(24, u32, 0u32, this + COUNT);
        }
        wr8(this + FLAGS2, rd8(this + FLAGS2) & 0xfd);
        wr8(this + FLAGS2, rd8(this + FLAGS2) & 0xfe);
        let b7 = f32::from_bits(buf[6]);
        wr32(this + COUNT, 0);
        wr32(this + 0xd0, 0);
        wr32(this + 0x20, 0);
        wr32(this + 0x24, 0);
        wr32(this + 0x28, 0);
        unsafe { ((this + 0x2c) as *mut u32).write_unaligned(b7.to_bits()) };
        if fb == 0 {
            return rd32(this + LINK);
        }
        let t25: u32 = lf_checker_rt::callee_thiscall!(
            25, u32, lf_checker_rt::relocated(FACTORY_OBJ), 0u32
        );
        f10 = t25;
        let mut skip_setup = t25 == 0 || rd8(arg + 0x219) == 0;
        if !skip_setup {
            let u26: u32 = lf_checker_rt::callee_thiscall!(26, u32, t25);
            if (u26 as u8) == 0 {
                skip_setup = true;
            }
        }
        if !skip_setup {
            lf_checker_rt::callee_cdecl!(27, u32,);
            lf_checker_rt::callee_cdecl!(28, u32, 0x124u32, 0x3f800000u32);
        }
        let g29 = lf_checker_rt::global::<u32>(CREATOR).read();
        let x29: u32 = lf_checker_rt::callee_thiscall!(29, u32, g29);
        if x29 == 0 {
            fc = 0;
        } else {
            let f0 = rd8(this + FLAGS);
            // First argument reuses the caller's saved-esi slot, which a
            // rewrite cannot read; the contract skips it.
            fc = lf_checker_rt::callee_thiscall!(
                30, u32, x29, 0u32, arg, rd32(this + STATE),
                u32::from((f0 >> 6) & 1), 0u32, 0u32, 0u32
            );
        }
        let z31: u32 = lf_checker_rt::callee_thiscall!(31, u32, this, 1u32);
        fb = (z31 as u8) & 0xff;
        lf_checker_rt::callee_thiscall!(32, u32, buf.as_ptr() as u32 + 12);
        let ev = [
            lf_checker_rt::relocated(EVT_TAG),
            buf[4],
            buf[5],
            0u32,
            fc,
            0x4au32,
        ];
        if rd8(t8 + 0x0c) & 1 == 0 {
            let hk: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                unsafe { core::mem::transmute(rd32(rd32(t8) + 0x14) as usize) };
            if (hk(t8, arg, 1u32, ev.as_ptr() as u32) as u8) != 0 {
                wr32(t8 + 0x0c, rd32(t8 + 0x0c) | 2);
            }
        }
        if rd8(arg + 0x219) == 0 {
            let hi = rd32(lf_checker_rt::relocated(G_COUNT_HI)) as i32;
            let lo = rd32(lf_checker_rt::relocated(G_COUNT_LO)) as i32;
            let n: u32 = lf_checker_rt::callee_cdecl!(33, u32,);
            let c = buf[2];
            let m = ((n & 0xffff) as i32) as f32;
            let d = hi.wrapping_sub(lo);
            let k = f32::from_bits(rd32(lf_checker_rt::relocated(K_STEP)));
            let mut f1 = m;
            f1 = mul(f1, k);
            let mut f0 = d as f32;
            f0 = mul(f0, f1);
            wr32(c + 0x44, rd32(lf_checker_rt::relocated(G_STAMP)));
            let t = cvtt(f0);
            wr8(c + 0x4c, 1);
            wr32(c + 0x48, (t as u32).wrapping_add(lo as u32));
        }
        lf_checker_rt::callee_thiscall!(34, u32, this, arg, f10, 1u32, 0u32);
        lf_checker_rt::callee_thiscall!(35, u32, this, arg);
        if f10 == 0 {
            wr8(this + FLAGS, rd8(this + FLAGS) & 0xfb);
        } else {
            let old = rd8(this + FLAGS);
            let mut al = u32::from(fb == 0) as u8;
            al <<= 2;
            al ^= old;
            al &= 4;
            wr8(this + FLAGS, old ^ al);
        }
        lf_checker_rt::callee_thiscall!(36, u32, ev.as_ptr() as u32);
        fc
    }
});
