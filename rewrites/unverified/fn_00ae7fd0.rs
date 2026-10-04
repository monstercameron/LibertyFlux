// original: 0x00AE7FD0 filter_timing_range (proposed)

/// Sweep a range of timed entries in two passes: retire stale ones, then
/// score the survivors by range.
///
/// Arguments (cdecl): `a0` is passed through to the retire callee; `start`
/// and `end` delimit an array of 8-byte entries (object pointer, float
/// tag); `ctx` is the timing context. The context bit selector at `+0x900`
/// picks the cleared bit, `+0x8E8` bit 28 feeds the retire callee, and the
/// floats at `+0x910`/`+0x914`/`+0x918` are the origin the ranges are
/// measured from. Returns nothing meaningful (leftover EAX; `ret: none`).
///
/// Pass one runs only when the global pass flag is set. For each live entry
/// whose `+0x24` bit 5 is clear, the context bit is cleared from the mask
/// words at `+0x54`/`+0x58` and the entry is nulled. Every live entry is
/// then polled through callee 1 (the class row
/// `CLASS_TABLE[kind at +0x2E]`, virtual slot `+0xC`); an answer of 2
/// retires it through callee 2 `(object, bit, a0, ctx, 1)`, clears the
/// context bit from the word at `+0x7C` and nulls the entry.
///
/// Pass two scores each live entry. When the pass flag is clear the entry
/// is reported unconditionally through callee 4 `(object, tag, uncovered,
/// ctx)` where `uncovered` is the newly seen bits `~+0xC & +0x8`. When set,
/// `base` (`+0x54` low 24) and `vis` (`+0x58` low 24, forced to 0 while the
/// global visibility flag is clear) gate a range test: if both are nonzero
/// and `+0x24` bit 7 is set, callee 3 (virtual slot `+0x5C`) yields a point
/// and a radius, and `near` means the distance from the origin minus the
/// radius is below 100. Near with the global hold flag clear and `+0x5C`
/// bit 14 clear drops `vis` to 0; far above 200 clears `+0x5C` bit 14.
/// A second range test against 20 then masks `vis` down unless the global
/// second mask is 0, `base` is 0, the masks are disjoint, or `+0x24` bit 7
/// is set.
/// `uncovered` becomes `base & ~vis`, forced to 0 while state `+0x28` bit
/// 19 is set; a zero value clears `+0x5C` bit 13. While that bit is set and
/// the global level mask touches `vis`, the level byte `+0x63` counts down
/// by 0x11 (hitting 0 also clears the bit) and `uncovered` is dropped.
/// `+0x5C` bit 14 is then set when the global mode word is set, `+0x24`
/// bit 7 is set and state bit 25 is clear, else cleared when `vis` is 0.
/// While the level mask touches `vis` the level is pinned to 0xFF and
/// `+0x5C` bit 13 is set unless the entry and its parent (at `+0x4C`)
/// clear a chain of mode, state-bit-25 and `+0x24`-bit-7 guards.
/// `uncovered` is folded into `+0xC`; the recomputed uncovered bits raise
/// the level by 0x10 (saturating at 0xFF past 0xEF) unless the level mask
/// touches them. A nonzero result is reported through callee 4, and the
/// low 24 bits of both mask words are cleared. The float operation order
/// is the original's, pinned through `black_box` helpers.
///
/// The class row for an object is `CLASS_TABLE[word at +0x2E]`, indexed by
/// the sign-extended 16-bit kind. Original: 0x00AE7FD0 (cdecl, four stack
/// words).
lf_checker_rt::export!(cdecl, rw_00AE7FD0(a0: u32, start: u32, end: u32, ctx: u32) -> u32 {
    unsafe {
        const CLASS_TABLE: u32 = 0x01295CD8;
        const PASS_FLAG: u32 = 0x0103F720;
        const VIS_FLAG: u32 = 0x0103F6E6;
        const TOUCHED: u32 = 0x015DBDA4;
        const HOLD_FLAG: u32 = 0x01593BD0;
        const MASK2: u32 = 0x0159AF2C;
        const LEVEL_MASK: u32 = 0x0159AF28;
        const MODE_WORD: u32 = 0x016DD67C;
        const NEAR: f32 = 100.0;
        const FAR: f32 = 200.0;
        const SECOND: f32 = 20.0;
        const LOW24: u32 = 0x00FF_FFFF;
        const KIND: u32 = 0x2E;
        const OPTS: u32 = 0x24;
        const STATE: u32 = 0x28;
        const SEEN: u32 = 0x0C;
        const BITS: u32 = 0x08;
        const PARENT: u32 = 0x4C;
        const MASK_A: u32 = 0x54;
        const MASK_B: u32 = 0x58;
        const FLAGS: u32 = 0x5C;
        const LEVEL: u32 = 0x63;
        const RETIRE_BITS: u32 = 0x7C;
        const CTX_BITSEL: u32 = 0x900;
        const CTX_BIT: u32 = 0x8E8;
        const CTX_X: u32 = 0x910;
        const CTX_Y: u32 = 0x914;
        const CTX_Z: u32 = 0x918;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Distance from the context origin to callee 3's point for `obj`,
        /// minus its radius. One stub call yields both, as in the original.
        #[inline(always)]
        unsafe fn range_minus_radius(obj: u32, ctx: u32) -> f32 {
            unsafe {
                let range_fn: extern "thiscall" fn(u32, u32) -> f32 =
                    core::mem::transmute(rd32(rd32(obj) + 0x5C) as usize);
                let mut p = [0u32; 3];
                let radius = range_fn(obj, p.as_mut_ptr() as u32);
                let vx = f32::from_bits(p[0]);
                let vy = f32::from_bits(p[1]);
                let vz = f32::from_bits(p[2]);
                let dx = sub(vx, f32::from_bits(rd32(ctx + CTX_X)));
                let dy = sub(vy, f32::from_bits(rd32(ctx + CTX_Y)));
                let dz = sub(vz, f32::from_bits(rd32(ctx + CTX_Z)));
                let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                sub(d2.sqrt(), radius)
            }
        }

        let table = lf_checker_rt::relocated(CLASS_TABLE);
        let pass = lf_checker_rt::global::<u8>(PASS_FLAG).read();
        let bitsel = rd32(ctx + CTX_BITSEL);
        lf_checker_rt::global::<u32>(TOUCHED).write_unaligned(0);
        let class_of = |obj: u32| unsafe {
            let idx = ((obj + KIND) as *const i16).read_unaligned() as i32;
            (table as *const u32).offset(idx as isize).read_unaligned()
        };
        if pass != 0 {
            let mut cur = start;
            while cur != end {
                let obj = rd32(cur);
                if obj != 0 {
                    if rd8(obj + OPTS) & 0x20 == 0 {
                        let keep = !(1u32.wrapping_shl(bitsel & 31)) | 0xFF00_0000;
                        wr32(obj + MASK_A, rd32(obj + MASK_A) & keep);
                        wr32(obj + MASK_B, rd32(obj + MASK_B) & keep);
                        wr32(cur, 0);
                    }
                    let class = class_of(obj);
                    let poll: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(rd32(class) + 0x0C) as usize);
                    if poll(class) & 0xFF == 2 {
                        let b = (rd32(ctx + CTX_BIT) >> 0x1C) & 1;
                        let _: u32 = lf_checker_rt::callee_cdecl!(2, u32, obj, b, a0, ctx, 1);
                        // 16-bit shift of 1 by the 5-bit-masked count.
                        let ax = (1u32.wrapping_shl(bitsel & 31) & 0xFFFF) as u16;
                        wr16(obj + RETIRE_BITS, rd16(obj + RETIRE_BITS) & !ax);
                        wr32(cur, 0);
                    }
                }
                cur = cur.wrapping_add(8);
            }
        }
        let mut cur = start;
        while cur != end {
            let obj = rd32(cur);
            if obj != 0 {
                if pass == 0 {
                    let uncovered = !rd32(obj + SEEN) & rd32(obj + BITS);
                    let tag = rd32(cur + 4);
                    let _: u32 = lf_checker_rt::callee_cdecl!(4, u32, obj, tag, uncovered, ctx);
                } else {
                    let base = rd32(obj + MASK_A) & LOW24;
                    let mut vis = rd32(obj + MASK_B) & LOW24;
                    if lf_checker_rt::global::<u8>(VIS_FLAG).read() == 0 {
                        vis = 0;
                    }
                    let g = lf_checker_rt::global::<u32>;
                    if base != 0 && vis != 0 && rd8(obj + OPTS) & 0x80 != 0 {
                        let t = range_minus_radius(obj, ctx);
                        if NEAR > t {
                            if lf_checker_rt::global::<u8>(HOLD_FLAG).read() != 0
                                || rd16(obj + FLAGS) & 0x4000 != 0
                            {
                                // `base` survives; nothing else changes.
                            } else {
                                vis = 0;
                            }
                        } else if t > FAR {
                            wr16(obj + FLAGS, rd16(obj + FLAGS) & 0xBFFF);
                        }
                    }
                    let m2 = g(MASK2).read_unaligned();
                    if m2 != 0 && base != 0 && vis & m2 != 0 && rd8(obj + OPTS) & 0x80 == 0 {
                        let t2 = range_minus_radius(obj, ctx);
                        if SECOND > t2 {
                            vis &= !m2;
                        }
                    }
                    let mut uncovered = base & !vis;
                    let state = rd32(obj + STATE);
                    if (state >> 0x13) & 1 != 0 {
                        uncovered = 0;
                    }
                    if uncovered == 0 {
                        wr16(obj + FLAGS, rd16(obj + FLAGS) & 0xDFFF);
                    }
                    if rd16(obj + FLAGS) & 0x2000 != 0
                        && g(LEVEL_MASK).read_unaligned() & vis != 0
                    {
                        let down = rd8(obj + LEVEL).wrapping_sub(0x11);
                        uncovered = 0;
                        if down as i8 <= 0 {
                            wr16(obj + FLAGS, rd16(obj + FLAGS) & 0xDFFF);
                            wr8(obj + LEVEL, 0);
                        } else {
                            wr8(obj + LEVEL, down);
                        }
                    }
                    let parent = rd32(obj + PARENT);
                    if g(MODE_WORD).read_unaligned() != 0
                        && rd8(obj + OPTS) & 0x80 != 0
                        && (state >> 0x19) & 1 == 0
                    {
                        wr16(obj + FLAGS, rd16(obj + FLAGS) | 0x4000);
                    } else if vis == 0 {
                        wr16(obj + FLAGS, rd16(obj + FLAGS) & 0xBFFF);
                    }
                    if g(LEVEL_MASK).read_unaligned() & vis != 0 {
                        wr8(obj + LEVEL, 0xFF);
                        let pbit25 =
                            parent != 0 && (rd32(parent + STATE) >> 0x19) & 1 != 0;
                        let pbit80 = parent != 0 && rd8(parent + OPTS) & 0x80 != 0;
                        let set = if g(MODE_WORD).read_unaligned() == 0 {
                            true
                        } else if pbit25 && pbit80 {
                            true
                        } else {
                            let b7 = (rd8(obj + OPTS) >> 7) & 1;
                            (b7 == 0 && parent == 0)
                                || (b7 != 0 && (state >> 0x19) & 1 != 0)
                        };
                        if set {
                            wr16(obj + FLAGS, rd16(obj + FLAGS) | 0x2000);
                        }
                    }
                    wr32(obj + SEEN, rd32(obj + SEEN) | uncovered);
                    let fresh = !rd32(obj + SEEN) & rd32(obj + BITS);
                    if g(LEVEL_MASK).read_unaligned() & fresh == 0 {
                        let lv = rd8(obj + LEVEL);
                        if lv >= 0xEF {
                            wr8(obj + LEVEL, 0xFF);
                        } else {
                            wr8(obj + LEVEL, lv.wrapping_add(0x10));
                        }
                    }
                    if fresh != 0 {
                        let tag = rd32(cur + 4);
                        let _: u32 =
                            lf_checker_rt::callee_cdecl!(4, u32, obj, tag, fresh, ctx);
                    }
                    wr32(obj + MASK_A, rd32(obj + MASK_A) & 0xFF00_0000);
                    wr32(obj + MASK_B, rd32(obj + MASK_B) & 0xFF00_0000);
                }
            }
            cur = cur.wrapping_add(8);
        }
        0
    }
});
