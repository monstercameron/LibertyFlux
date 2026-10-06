// original: 0x00b32ef0 peds_task_area_update (proposed)

/// Refresh the task area covering a query point, then run its cells.
///
/// `obj` points at the query record (three floats at `+0x00`/`+0x04`/`+0x08`
/// and a radius bias at `+0x10`). The prologue maps `(x-c)`, `(y-c)`,
/// `(x+c)`, `(y+c)` through a fixed scale and offset (0.02 and 60.0) and a
/// round-down quantizer to four cell bounds: inner from/to and outer
/// from/to. A generation word in writable game data is bumped on every call;
/// when it has reached 0xFFFF the callee 5 reset runs instead and the word
/// restarts at 1. An empty outer range returns at once (0xFFFF on the bump
/// path, 1 on the reset path).
///
/// Each covered cell `(i, j)` addresses a 20-byte row in table 1
/// (`(i & 15) + ((j & 15) << 4)`) and a 12-byte row in table 2 (indices
/// clamped at 0 and 0x77). When bit 2 of `earg` is set, up to seven
/// conditional calls to callee 1 fire, gated by bits 1, 2, 3, 4, 5, 7 and 8
/// of `darg`, each answered 0 meaning abort with that answer. When bits 3-4
/// of `earg` select the list walk, the head stored at table-1-row+0x10 is
/// walked (nodes hold an entry pointer and a next pointer): on the measured
/// path each live entry (bit 0x4000000 at entry+0x28) yields a position
/// through vtable slot +0x54 and a height through vtable slot +0x58, and the
/// squared radius `(height + bias)^2` must strictly exceed the squared
/// distance or the entry is skipped (an unordered comparison skips too);
/// entries whose stamp at +0x3c differs from the generation word are
/// restamped and passed to callee 4 with `(obj, barg, w10, darg)`, whose
/// zero answer aborts. The return value is the final inner latch value on
/// completion, or the aborting callee's full answer on an early exit.
///
/// Float order matches the original operation for operation (lane 0 of every
/// vector step; upper lanes stay zero and never reach memory), so integer
/// truncation after a non-exact scale rounds identically, and the
/// float-to-int step reproduces truncate-with-INT_MIN-on-invalid exactly.
///
/// Original: 0x00b32ef0 (cdecl, five stack words).
#[inline(always)]
unsafe fn area_update<const MUT: bool>(obj: u32, barg: u32, w10: u32, darg: u32, earg: u32) -> u32 {
    unsafe {
        const K02_VA: u32 = 0x00FE8734;
        const K60_VA: u32 = 0x00FE8B80;
        const K2P23_BITS: u32 = 0x4B000000;
        const SIGN_BITS: u32 = 0x80000000;
        const K1_BITS: u32 = 0x3F800000;
        const TAB1_VA: u32 = 0x011A8918;
        const TAB2_VA: u32 = 0x011A9D20;
        const FLAGW_VA: u32 = 0x011A8908;
        const CLAMP: i32 = 0x77;
        const WRAP_AT: u16 = 0xFFFF;
        const SCAN_CALLEE: u32 = 1;
        const ENTER_CALLEE: u32 = 4;
        const RESET_CALLEE: u32 = 5;
        const VT_POS: u32 = 0x54;
        const VT_H: u32 = 0x58;
        const ALIVE_BIT: u32 = 0x0400_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Truncate toward zero, matching the convert-with-truncation
        /// instruction: NaN and out-of-range magnitudes yield INT_MIN.
        #[inline(always)]
        fn cvtt(v: f32) -> i32 {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 {
                i32::MIN
            } else {
                v as i32
            }
        }
        /// The round-down quantizer shared by all four bounds: add and
        /// subtract a sign-matched power of two (a no-op above 2^23, where
        /// the offset collapses to signed zero), then subtract 1.0 unless
        /// the rounded value came in below the input.
        #[inline(always)]
        fn round_coord(v: f32, k2p23: f32) -> f32 {
            let sign = f32::from_bits(v.to_bits() & SIGN_BITS);
            let av = f32::from_bits(v.to_bits() ^ sign.to_bits());
            let m0 = if av < k2p23 { 0xFFFF_FFFFu32 } else { 0 };
            let m1 = f32::from_bits((K2P23_BITS & m0) | sign.to_bits());
            let r = sub(add(v, m1), m1);
            let e = sub(r, v);
            let m2 = if e < sign { 0u32 } else { 0xFFFF_FFFF };
            let adj = f32::from_bits(m2 & K1_BITS);
            sub(r, adj)
        }

        let k02 = f32::from_bits(rd32(lf_checker_rt::relocated(K02_VA)));
        let k60 = f32::from_bits(rd32(lf_checker_rt::relocated(K60_VA)));
        let k2p23 = f32::from_bits(K2P23_BITS);
        let x = rdf(obj);
        let y = rdf(obj.wrapping_add(4));
        let cc = rdf(obj.wrapping_add(0x10));
        let inner_from = cvtt(round_coord(add(mul(sub(x, cc), k02), k60), k2p23));
        let outer_from = cvtt(round_coord(add(mul(sub(y, cc), k02), k60), k2p23));
        let inner_to = cvtt(round_coord(add(mul(add(x, cc), k02), k60), k2p23));
        let outer_to = cvtt(round_coord(add(mul(add(y, cc), k02), k60), k2p23));

        let flagp = lf_checker_rt::global::<u16>(FLAGW_VA);
        let flag = (flagp as *const u16).read_unaligned();
        let mut eax: u32;
        if flag >= WRAP_AT {
            let _reset: u32 = lf_checker_rt::callee_cdecl!(RESET_CALLEE, u32);
            flagp.write_unaligned(1);
            eax = 1;
        } else {
            flagp.write_unaligned(flag.wrapping_add(1));
            eax = 0xFFFF;
        }

        let exit_early = if MUT { outer_from >= outer_to } else { outer_from > outer_to };
        if exit_early {
            return eax;
        }

        let t1 = lf_checker_rt::relocated(TAB1_VA);
        let t2 = lf_checker_rt::relocated(TAB2_VA);
        let mut jcur = outer_from;
        loop {
            eax = inner_from as u32;
            if inner_from <= inner_to {
                let jlow = ((jcur as u32) & 0xF).wrapping_shl(4);
                let do_sites = earg & 4 != 0;
                let mut icur = inner_from;
                loop {
                    let ci = if icur > 0 { icur } else { 0 };
                    let cj = if jcur > 0 { jcur } else { 0 };
                    let cci = if ci < CLAMP { ci } else { CLAMP };
                    let ccj = if cj < CLAMP { cj } else { CLAMP };
                    let cell = (cci as u32)
                        .wrapping_add((ccj as u32).wrapping_mul(120))
                        .wrapping_mul(3);
                    let t2ptr = t2.wrapping_add(cell.wrapping_mul(4));
                    let row = ((icur as u32) & 0xF).wrapping_add(jlow);
                    let t1ptr = t1.wrapping_add(row.wrapping_mul(20));
                    if do_sites {
                        if darg & 0x02 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(SCAN_CALLEE, u32, obj, t2ptr, barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x04 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(SCAN_CALLEE, u32, obj, t1ptr, barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x08 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(
                                SCAN_CALLEE, u32, obj, t1ptr.wrapping_add(4), barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x10 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(
                                SCAN_CALLEE, u32, obj, t1ptr.wrapping_add(8), barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x20 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(
                                SCAN_CALLEE, u32, obj, t2ptr.wrapping_add(4), barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x80 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(
                                SCAN_CALLEE, u32, obj, t1ptr.wrapping_add(0xC), barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                        if darg & 0x100 != 0 {
                            let a = lf_checker_rt::callee_cdecl!(
                                SCAN_CALLEE, u32, obj, t1ptr.wrapping_add(0x10), barg, w10);
                            if a as u8 == 0 {
                                return a;
                            }
                        }
                    }
                    if earg & 0x18 != 0 {
                        let mut node = rd32(t1ptr.wrapping_add(0x10));
                        if node != 0 {
                            let use_measure = earg & 0x10 != 0;
                            loop {
                                let ent = rd32(node);
                                node = rd32(node.wrapping_add(4));
                                let mut reach_stamp = true;
                                if use_measure {
                                    if rd32(ent.wrapping_add(0x28)) & ALIVE_BIT == 0 {
                                        reach_stamp = false;
                                    } else {
                                        let mut scratch = [0u32; 3];
                                        let pos_of: extern "thiscall" fn(u32, u32) -> u32 =
                                            core::mem::transmute(
                                                rd32(rd32(ent).wrapping_add(VT_POS)) as usize);
                                        let pp = pos_of(ent, scratch.as_mut_ptr() as u32);
                                        let dx = sub(rdf(obj), rdf(pp));
                                        let dy = sub(rdf(obj.wrapping_add(4)), rdf(pp.wrapping_add(4)));
                                        let dz = sub(rdf(obj.wrapping_add(8)), rdf(pp.wrapping_add(8)));
                                        let height_of: extern "thiscall" fn(u32) -> f32 =
                                            core::mem::transmute(
                                                rd32(rd32(ent).wrapping_add(VT_H)) as usize);
                                        let h = height_of(ent);
                                        let r = add(h, cc);
                                        let r2 = mul(r, r);
                                        let dist =
                                            add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                                        if !(r2 > dist) {
                                            reach_stamp = false;
                                        }
                                    }
                                }
                                if reach_stamp {
                                    let now = (flagp as *const u16).read_unaligned() as u32;
                                    if rd32(ent.wrapping_add(0x3c)) != now {
                                        wr32(ent.wrapping_add(0x3c), now);
                                        let a = lf_checker_rt::callee_thiscall!(
                                            ENTER_CALLEE, u32, ent, obj, barg, w10, darg);
                                        if a as u8 == 0 {
                                            return a;
                                        }
                                    }
                                }
                                if node == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    icur = icur.wrapping_add(1);
                    eax = icur as u32;
                    if icur > inner_to {
                        break;
                    }
                }
            }
            jcur = jcur.wrapping_add(1);
            if jcur > outer_to {
                break;
            }
        }
        eax
    }
}

lf_checker_rt::export!(cdecl, rw_00b32ef0(obj: u32, barg: u32, w10: u32, darg: u32, earg: u32) -> u32 {
    unsafe { area_update::<false>(obj, barg, w10, darg, earg) }
});
