// original: 0x00CCD5D0 ped_task_setup (proposed)

/// Set up a ped's task state: validate preconditions, build a working
/// frame, refresh two smoothed scalars.
///
/// `this` is the task owner, `edi` the ped. When the word at `this+0x10`
/// is non-zero the whole middle is skipped. Otherwise a manager object
/// (from the `MGR_SLOT` global) yields a list whose entries are each
/// offered, together with six constant ids (0xBF, 0xC0, 0x171-0x174), to
/// a per-id check: the first non-zero answer diverts to the exit-A path,
/// which reports the failing entry's `+0xC`/`+0x10` words and its `+0x4C`
/// float and returns to the tail.
///
/// On the normal path a flag (`[edi+0xF4]` bit 0x40) selects a frame
/// section: three small constructors run (the third only when
/// `[this+0x18]` bit 0x40 is clear), each followed by a filler call that
/// writes one word into the frame; that word's low half-word gates an
/// indirect-call block (object slot 0xA0, then slot 0xE0 of its result)
/// feeding a seven-struct call, and its high half-word gates one scalar
/// call. Three frame words filled by the seven-struct call are each
/// tested against +0.0 (an unordered NaN counts as different, like
/// `!=`): any difference takes the second report call, otherwise the
/// first one runs with the frame's id pair.
///
/// The tail clears bit 15 of a flags word (only reachable with a null
/// holder, so the write itself never fires), optionally refreshes a
/// cached object through slots 0xB0/0xA4/0x10 when the ped's state words
/// allow, and, unless disabled, evaluates two dot products of a matrix
/// row (at `[edi+0x20]`) with the ped's `+0xB00` vector, clamps each to
/// [-1, 1] (a NaN passes through), maps them through one float-in-xmm0
/// helper and stores the results at `+0xC00`/`+0xBF8`, setting bit 1 at
/// `+0xBE0`. All float arithmetic keeps the original's operand order.
///
/// Original: 0x00CCD5D0 (thiscall, one stack word; the caller ignores EAX).
lf_checker_rt::export!(thiscall, rw_00ccd5d0(this: u32, edi: u32) -> u32 {
    unsafe {
        const MGR_SLOT: u32 = 0x016DD63C;
        const CLAMP_LO: u32 = 0x00FE8D94; // -1.0
        const CLAMP_HI: u32 = 0x00FE88E8; // +1.0
        const D_LIST: u32 = 1;
        const D_CHECK: u32 = 2;
        const D_CTOR: u32 = 3;
        const D_FILL: u32 = 4;
        const D_COMBINE: u32 = 5;
        const D_SCALAR: u32 = 6;
        const D_REPORT2: u32 = 7;
        const D_REPORT1: u32 = 8;
        const D_FLOAT: u32 = 9;
        const D_MAP: u32 = 10;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn slot0(obj: u32, off: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(off)) as usize);
                f(obj)
            }
        }

        // Tail (runs on every path): bit clear, cache refresh, FP finish.
        // Written as straight-line code after the middle; the middle sets
        // nothing it needs.
        if rd32(this.wrapping_add(0x10)) == 0 {
            // Middle: list + checks, exit-A or frame section.
            let mgr = rd32(lf_checker_rt::relocated(MGR_SLOT));
            let plist: u32 = lf_checker_rt::callee_thiscall!(D_LIST, u32, mgr, 0x2C);
            let mut failed: u32 = 0;
            let mut esi = 0i32;
            if (rd32(plist.wrapping_add(0x38)) as i32) > 0 {
                let mut off = 0u32;
                loop {
                    let base = rd32(plist.wrapping_add(0x44));
                    let v = rd32(base.wrapping_add(off));
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        D_CHECK, u32, rd32(edi.wrapping_add(0x78)), v
                    );
                    if r != 0 {
                        failed = r;
                        break;
                    }
                    off = off.wrapping_add(12);
                    esi += 1;
                    if !(esi < rd32(plist.wrapping_add(0x38)) as i32) {
                        break;
                    }
                }
            }
            if failed == 0 {
                for id in [0xBFu32, 0xC0, 0x171, 0x172, 0x173, 0x174] {
                    let r: u32 = lf_checker_rt::callee_thiscall!(
                        D_CHECK, u32, rd32(edi.wrapping_add(0x78)), id
                    );
                    if r != 0 {
                        failed = r;
                        break;
                    }
                }
            }
            if failed != 0 {
                // Exit A: report the failing entry.
                let fbits = rd32(failed.wrapping_add(0x4C));
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    D_REPORT1, u32, this, edi,
                    rd32(failed.wrapping_add(0x10)),
                    rd32(failed.wrapping_add(0x0C)),
                    0x4100_0000, 1
                );
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    D_FLOAT, u32, rd32(this.wrapping_add(0x10)), fbits
                );
            } else if rd8(edi.wrapping_add(0xF4)) & 0x40 == 0 {
                // Frame section skipped: id pair was already set in registers (0xBF/0x2E) before the skip test.
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    D_REPORT1, u32, this, edi, 0x2E, 0xBF, 0x4100_0000, 2
                );
            } else {
                // Frame section. frame[i] models the original's
                // [esp+8+i*4] words the proof observes.
                let mut frame = [0u32; 24];
                frame[1] = 0xBF;
                frame[2] = 0x2E;
                frame[4] = 0;
                frame[5] = 0;
                frame[6] = 0x3F80_0000;
                frame[7] = 0;
                frame[8] = 0;
                frame[10] = 0;
                frame[11] = 0x3F80_0000;
                frame[12] = 0;
                frame[14] = 0;
                frame[15] = 0;
                frame[16] = 0x3F80_0000;
                frame[18] = 0;
                frame[19] = 0;
                frame[20] = 0;
                let r1: u32 = lf_checker_rt::callee_cdecl!(D_CTOR, u32, 0x2E, 0xC0, 0);
                let p1: u32 = lf_checker_rt::callee_thiscall!(
                    D_FILL, u32, frame.as_mut_ptr().add(4) as u32, 0x10
                );
                wr32(p1, r1);
                let r2: u32 = lf_checker_rt::callee_cdecl!(D_CTOR, u32, 0x2E, 0xBF, 0);
                let p2: u32 = lf_checker_rt::callee_thiscall!(
                    D_FILL, u32, frame.as_mut_ptr().add(4) as u32, 0x10
                );
                wr32(p2, r2);
                if rd8(this.wrapping_add(0x18)) & 0x40 == 0 {
                    let r3: u32 =
                        lf_checker_rt::callee_cdecl!(D_CTOR, u32, 0x2C, 0xBE, 0);
                    let p3: u32 = lf_checker_rt::callee_thiscall!(
                        D_FILL, u32, frame.as_mut_ptr().add(4) as u32, 0x10
                    );
                    wr32(p3, r3);
                }
                let w0 = frame[5];
                if (w0 & 0xFFFF) != 0 {
                    frame[3] = 0;
                    frame[0] = 0;
                    let q1 = slot0(edi, 0xA0);
                    let d5a0 = if q1 == 0 {
                        rd32(edi.wrapping_add(0x100))
                    } else {
                        let q2 = slot0(edi, 0xA0);
                        slot0(q2, 0xE0)
                    };
                    let p = frame.as_mut_ptr();
                    let _: u32 = lf_checker_rt::callee_cdecl!(
                        D_COMBINE, u32, d5a0,
                        p.add(4) as u32, p.add(2) as u32, p.add(1) as u32,
                        p.add(3) as u32, p as u32, p.add(6) as u32
                    );
                }
                if ((w0 >> 16) & 0xFFFF) != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_cdecl!(D_SCALAR, u32, frame[4]);
                }
                let x50 = f32::from_bits(frame[18]);
                let x54 = f32::from_bits(frame[19]);
                let x58 = f32::from_bits(frame[20]);
                if x50 != 0.0 || x54 != 0.0 || x58 != 0.0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        D_REPORT2, u32, this, edi, 0x2E, 0xBF, 0x4100_0000,
                        frame.as_ptr().add(6) as u32, 2
                    );
                } else {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        D_REPORT1, u32, this, edi, frame[2], frame[1],
                        0x4100_0000, 2
                    );
                }
            }
            // Flag-word clear; the holder is null on every reaching path
            // (a non-null holder skips the middle), so only the check runs.
            let edx = rd32(this.wrapping_add(0x10));
            if edx != 0 {
                let c = rd32(edx.wrapping_add(4));
                if (c >> 15) & 1 != 0 {
                    wr32(edx.wrapping_add(4), c & 0xFFFF_7FFF);
                }
            }
        }
        // Cache refresh through object slots.
        {
            let a38 = rd32(edi.wrapping_add(0x38));
            let mut go = a38 != 0;
            if go {
                if rd16(a38.wrapping_add(8)) == 0xFFFF {
                    go = false;
                } else if rd8(edi.wrapping_add(0x26C)) & 1 == 0 {
                    go = false;
                } else if rd32(edi.wrapping_add(0xAB0)) != 0 {
                    go = false;
                } else if rd8(edi.wrapping_add(0x29C)) & 4 != 0 {
                    go = false;
                }
            }
            if go {
                let _: u32 = slot0(edi, 0xB0);
                let s1 = slot0(edi, 0xA4);
                if s1 != 0 {
                    let s2 = slot0(edi, 0xA4);
                    let _: u32 = slot0(s2, 0x10);
                }
            }
        }
        // Smoothed-scalar finish.
        {
            let st = rd8(edi.wrapping_add(0x26C));
            let go = if st & 4 != 0 {
                false
            } else if st & 1 != 0 {
                true
            } else {
                rd8(edi.wrapping_add(0x24)) & 1 == 0
            };
            if go {
                let m = rd32(edi.wrapping_add(0x20));
                let b00 = f32::from_bits(rd32(edi.wrapping_add(0xB00)));
                let b04 = f32::from_bits(rd32(edi.wrapping_add(0xB04)));
                let b08 = f32::from_bits(rd32(edi.wrapping_add(0xB08)));
                let mut x0 = f32::from_bits(rd32(m.wrapping_add(0x10)));
                let mut x3 = b00;
                let mut x4 = f32::from_bits(rd32(m.wrapping_add(0x14)));
                x0 = mul(x0, x3);
                x3 = mul(x3, f32::from_bits(rd32(m)));
                x4 = mul(x4, b04);
                x4 = add(x4, x0);
                x0 = f32::from_bits(rd32(m.wrapping_add(0x18)));
                x0 = mul(x0, b08);
                x4 = add(x4, x0);
                let save = x4;
                x0 = f32::from_bits(rd32(m.wrapping_add(8)));
                x0 = mul(x0, b08);
                x4 = f32::from_bits(rd32(m.wrapping_add(4)));
                x4 = mul(x4, b04);
                x4 = add(x4, x3);
                x4 = add(x4, x0);
                let c1 = f32::from_bits(rd32(lf_checker_rt::relocated(CLAMP_LO)));
                let c2 = f32::from_bits(rd32(lf_checker_rt::relocated(CLAMP_HI)));
                // Clamp to [c1, c2]; the original's third compare is dead
                // (it re-tests an unchanged value) and NaN passes through.
                let mut x = x4;
                if c1 > x {
                    x = c1;
                } else if x > c2 {
                    x = c2;
                }
                let r1: u32 =
                    lf_checker_rt::callee_cdecl!(D_MAP, u32, x.to_bits());
                wr32(edi.wrapping_add(0xC00), r1);
                let mut a = c1;
                if !(c1 > save) {
                    if save > c2 {
                        a = c2;
                    } else {
                        a = save;
                    }
                }
                let r2: u32 =
                    lf_checker_rt::callee_cdecl!(D_MAP, u32, a.to_bits());
                wr32(edi.wrapping_add(0xBF8), r2);
                wr32(
                    edi.wrapping_add(0xBE0),
                    rd32(edi.wrapping_add(0xBE0)) | 2,
                );
            }
        }
        0
    }
});
