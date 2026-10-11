// original: 0x00b3f640 task_target_select (proposed)

/// Pick a task target for a ped by random sampling, or by shuffling four
/// candidate boxes and recursing into the first contained one.
///
/// Main path: `a1 + 0x2c` points at a sample set (threshold byte at `+2`,
/// entry count at `+0xc`, u16 entry-index array at `+0x4`). A random start
/// cursor is drawn (`trunc(rand01 * count)` with `rand01 = (rand & 0xffff)
/// * 2^-15`), then each of the count entries is visited once: the u16
/// selects a 40-byte entry from `a0 + 0x6c`, entries with bit 13 of their
/// first word set are skipped, as are entries whose level field (bits
/// 29-31 of the second word, as a float) falls below the global threshold.
/// A surviving entry's position is resolved through the pose callee into
/// three floats, its squared distance to the global centre (formed as
/// `(dy*dy + dx*dx) + dz*dz`) must not exceed the outer radius, and three
/// distance bands select it: failing all bands rejects the entry unless a
/// global minimum is positive and the middle band holds.
///
/// An accepted entry either succeeds at once (when the global switch byte
/// is clear) or runs the probe chain: a one-time global initialisation, up
/// to two probe calls whose nonzero answers arm two hit flags, and three
/// gates combining the bands with the hits, ending in a final acceptance
/// call whose zero low byte means success.
///
/// Success expands the entry's fan-out field (bits 21-24 of its first
/// word) into that many slot callee calls, resolves two table rows through
/// picker/lookup callees (the second lookup takes the picker answer and
/// repeats, up to eleven calls, until its answer differs from the first),
/// blends the rows with a fresh random weight and publishes the blend into
/// three globals. The fourth published word comes from a stack slot the
/// original never writes: under the checker's defined stack fill that
/// value is 0, which is what this rewrite stores. Returns 1 on success, 0
/// when every entry is rejected.
///
/// Shuffle path (taken when `a1 + 0x2c` is null): the numbers 0..3 are
/// shuffled with the same random-draw shape (the CRT `rand` range keeps
/// every swap index in bounds) and each candidate box at `a1 + 0x30`
/// tested for containment of `a3` below and `a2` above; the first
/// contained box recurses into this same function. The checker answers the
/// recursive call from a script, so this proof covers one level only. The
/// original's security-cookie calls are reproduced as stub calls so the
/// call sequences match.
///
/// Original: 0x00B3F640 (cdecl, four stack words, boolean in AL).
lf_checker_rt::export!(cdecl, rw_00b3f640(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const THR: u32 = 0x016B7C4C;
        const BAND0: u32 = 0x016B7C38;
        const BAND1: u32 = 0x016B7C3C;
        const BAND2: u32 = 0x016B7C40;
        const BAND3: u32 = 0x016B7C44;
        const BANDMIN: u32 = 0x016B7C48;
        const CENTER: u32 = 0x016C8560;
        const SWITCH: u32 = 0x010459D9;
        const FLAGW: u32 = 0x0166540C;
        const FLAGF: u32 = 0x01665408;
        const TABIDX: u32 = 0x0118D818;
        const OUTBASE: u32 = 0x016C6730;
        const LERPTAB: u32 = 0x016C3CD0;
        const PUBX: u32 = 0x016C8570;
        const PUBY: u32 = 0x016C8574;
        const PUBZ: u32 = 0x016C8578;
        const PUBV: u32 = 0x016C857C;
        const K1: f32 = f32::from_bits(0x3800_0000); // 2^-15
        const K2: f32 = f32::from_bits(0x3800_0100);
        const ONE: f32 = 1.0;
        const TWO: f32 = 2.0;
        const CAL_RAND: u32 = 1;
        const CAL_POSE: u32 = 2;
        const CAL_PROBE1: u32 = 3;
        const CAL_PROBE2: u32 = 4;
        const CAL_FINAL: u32 = 5;
        const CAL_SLOT: u32 = 6;
        const CAL_PICK: u32 = 7;
        const CAL_LOOK1: u32 = 8;
        const CAL_LOOK2: u32 = 9;
        const CAL_COOKIE_OK: u32 = 10;
        const CAL_COOKIE_FAIL: u32 = 11;
        const CAL_SELF: u32 = 12;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn grd32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn grdf(va: u32) -> f32 {
            unsafe { f32::from_bits(grd32(va)) }
        }
        #[inline(always)]
        unsafe fn grd8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
        }
        #[inline(always)]
        unsafe fn gw32(va: u32, v: u32) {
            unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
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
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        /// Success expansion for a selected entry. Publishes the blend and
        /// returns 1.
        unsafe fn success(a0: u32, entry: u32) -> u32 {
            unsafe {
                const OUTBASE: u32 = 0x016C6730;
                const LERPTAB: u32 = 0x016C3CD0;
                const PUBX: u32 = 0x016C8570;
                const PUBY: u32 = 0x016C8574;
                const PUBZ: u32 = 0x016C8578;
                const PUBV: u32 = 0x016C857C;
                const K2: f32 = f32::from_bits(0x3800_0100);
                const ONE: f32 = 1.0;
                const CAL_RAND: u32 = 1;
                const CAL_SLOT: u32 = 6;
                const CAL_PICK: u32 = 7;
                const CAL_LOOK1: u32 = 8;
                const CAL_LOOK2: u32 = 9;
                const CAL_COOKIE_OK: u32 = 10;

                #[inline(always)]
                unsafe fn rd32(a: u32) -> u32 {
                    unsafe { (a as *const u32).read_unaligned() }
                }
                #[inline(always)]
                unsafe fn rd16(a: u32) -> u32 {
                    unsafe { (a as *const u16).read_unaligned() as u32 }
                }
                #[inline(always)]
                unsafe fn grdf(va: u32) -> f32 {
                    unsafe {
                        f32::from_bits(lf_checker_rt::global::<u32>(va).read_unaligned())
                    }
                }
                #[inline(always)]
                unsafe fn gw32(va: u32, v: u32) {
                    unsafe { lf_checker_rt::global::<u32>(va).write_unaligned(v) }
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
                fn sub(a: f32, b: f32) -> f32 {
                    core::hint::black_box(a) - core::hint::black_box(b)
                }

                let count2 = (rd32(entry) >> 21) & 0x0f;
                if count2 != 0 {
                    let base17 = rd32(entry.wrapping_add(4)) & 0x1_ffff;
                    let ubase = rd32(a0.wrapping_add(0x60));
                    let out0 = lf_checker_rt::relocated(OUTBASE);
                    let mut s = 0u32;
                    while s < count2 {
                        let u = rd16(ubase.wrapping_add(base17.wrapping_add(s).wrapping_mul(2)));
                        let _: u32 = lf_checker_rt::callee_thiscall!(
                            CAL_SLOT, u32, a0, u, out0.wrapping_add(s.wrapping_mul(0x10))
                        );
                        s = s.wrapping_add(1);
                    }
                }
                let v: u32 = lf_checker_rt::callee_cdecl!(
                    CAL_PICK, u32, entry, lf_checker_rt::relocated(OUTBASE), 0, 0, 1,
                    lf_checker_rt::relocated(LERPTAB)
                );
                let saved: u32 = lf_checker_rt::callee_cdecl!(CAL_LOOK1, u32, v, 0);
                let mut tries = 10u32;
                let mut cur = 0u32;
                loop {
                    // The picker answer reuses the fan-out slot here.
                    cur = lf_checker_rt::callee_cdecl!(CAL_LOOK2, u32, v, 0);
                    if cur != saved {
                        break;
                    }
                    if tries == 0 {
                        break;
                    }
                    tries -= 1;
                }
                let r: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,);
                let s6 = mul((r as i32) as f32, K2);
                let t = sub(ONE, s6);
                let ia = saved.wrapping_mul(16);
                let ib = cur.wrapping_mul(16);
                let x = add(mul(grdf(LERPTAB.wrapping_add(ia)), s6), mul(t, grdf(LERPTAB.wrapping_add(ib))));
                let y = add(
                    mul(grdf(LERPTAB.wrapping_add(ia).wrapping_add(4)), s6),
                    mul(grdf(LERPTAB.wrapping_add(ib).wrapping_add(4)), t),
                );
                let z = add(
                    mul(grdf(LERPTAB.wrapping_add(ia).wrapping_add(8)), s6),
                    mul(grdf(LERPTAB.wrapping_add(ib).wrapping_add(8)), t),
                );
                gw32(PUBX, x.to_bits());
                gw32(PUBY, y.to_bits());
                gw32(PUBZ, z.to_bits());
                // The original publishes an uninitialised stack slot here; the
                // contract defines that slot as 0 (stack_fill).
                gw32(PUBV, 0);
                let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_OK, u32,);
                1
            }
        }

        let sub0 = rd32(a1.wrapping_add(0x2c));
        if sub0 == 0 {
            // Shuffle path: shuffled containment tests with recursion.
            let mut arr = [0u32, 1u32, 2u32, 3u32];
            let mut s = 0u32;
            while s < 4 {
                let r: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) & 0xffff;
                let j = (mul(mul(r as f32, K1), (4 - s) as f32) as i32)
                    .wrapping_add(s as i32) as u32;
                let tmp = arr[s as usize];
                arr[s as usize] = arr[j as usize];
                arr[j as usize] = tmp;
                s += 1;
            }
            let mut k = 0u32;
            while k < 4 {
                let bx = rd32(a1.wrapping_add(arr[k as usize].wrapping_mul(4)).wrapping_add(0x30));
                if !(rdf(a3) >= rdf(bx)) {
                    k += 1;
                    continue;
                }
                if !(rdf(a3.wrapping_add(4)) >= rdf(bx.wrapping_add(4))) {
                    k += 1;
                    continue;
                }
                if !(rdf(bx.wrapping_add(0x10)) >= rdf(a2)) {
                    k += 1;
                    continue;
                }
                if !(rdf(bx.wrapping_add(0x14)) >= rdf(a2.wrapping_add(4))) {
                    k += 1;
                    continue;
                }
                let ans: u32 = lf_checker_rt::callee_cdecl!(CAL_SELF, u32, a0, bx, a2, a3);
                if (ans & 0xff) != 0 {
                    let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_OK, u32,);
                    return 1;
                }
                k += 1;
            }
            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_FAIL, u32,);
            return 0;
        }
        let count = rd16(sub0.wrapping_add(0x0c));
        if count == 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_FAIL, u32,);
            return 0;
        }
        let thr = grd32(THR);
        if (rd8(sub0.wrapping_add(2)) as u32) < thr {
            let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_FAIL, u32,);
            return 0;
        }
        let r0: u32 = lf_checker_rt::callee_cdecl!(CAL_RAND, u32,) & 0xffff;
        let mut esi = (mul(mul(r0 as f32, K1), count as f32) as i32);
        let mut ecx = count;
        loop {
            let eax = ecx;
            ecx = ecx.wrapping_sub(1);
            if eax == 0 {
                let _: u32 = lf_checker_rt::callee_stdcall!(CAL_COOKIE_FAIL, u32,);
                return 0;
            }
            esi = esi.wrapping_add(1);
            let smp = rd32(a1.wrapping_add(0x2c));
            // The index is read before it wraps; the wrap only affects the
            // next iteration.
            let edx = rd16(
                rd32(smp.wrapping_add(4))
                    .wrapping_add((esi as u32).wrapping_mul(2).wrapping_sub(2)),
            );
            if esi >= rd16(smp.wrapping_add(0x0c)) as i32 {
                esi = 0;
            }
            let entry =
                rd32(a0.wrapping_add(0x6c)).wrapping_add(edx.wrapping_mul(5).wrapping_mul(8));
            if ((rd32(entry) >> 13) & 1) != 0 {
                continue;
            }
            let bitval = (((rd32(entry.wrapping_add(4)) >> 29) & 7) as f32);
            if (thr as f32) > bitval {
                continue;
            }
            let mut pp = [0u32; 3];
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CAL_POSE, u32, a0, entry, pp.as_mut_ptr() as u32
            );
            let px = f32::from_bits(pp[0]);
            let py = f32::from_bits(pp[1]);
            let pz = f32::from_bits(pp[2]);
            let dx = sub(px, grdf(CENTER));
            let dy = sub(py, grdf(CENTER.wrapping_add(4)));
            let dz = sub(pz, grdf(CENTER.wrapping_add(8)));
            let d2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
            let g44 = grdf(BAND3);
            if d2 > g44 {
                continue;
            }
            let dl = d2 >= grdf(BAND0) && grdf(BAND1) >= d2;
            let cl = d2 >= grdf(BAND2) && g44 >= d2;
            let al = d2 >= grdf(BAND1) && grdf(BAND2) >= d2;
            if !dl && !cl {
                if !(grdf(BANDMIN) > 0.0) {
                    continue;
                }
                if !al {
                    continue;
                }
            }
            if grd8(SWITCH) == 0 {
                return success(a0, entry);
            }
            let mut ch = false;
            let mut cl2 = false;
            let flagw = grd32(FLAGW);
            if flagw & 1 == 0 {
                gw32(FLAGW, flagw | 1);
                gw32(FLAGF, add(grdf(BANDMIN), TWO).to_bits());
            }
            let tbi = grd32(TABIDX);
            let t = grd32(TABIDX.wrapping_add(tbi.wrapping_mul(4)));
            if t != 0 {
                let h1: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_PROBE1, u32, t.wrapping_add(0x10),
                    pp[0], pp[1], pp[2], TWO.to_bits(), 0
                );
                ch = h1 != 0;
            }
            if dl && ch {
                continue;
            }
            let t2 = grd32(TABIDX.wrapping_add(grd32(TABIDX).wrapping_mul(4)));
            if t2 != 0 {
                let h2: u32 = lf_checker_rt::callee_thiscall!(
                    CAL_PROBE2, u32, t2.wrapping_add(0x10),
                    pp[0], pp[1], pp[2], grd32(FLAGF), 0
                );
                cl2 = h2 != 0;
            }
            if cl && !cl2 {
                continue;
            }
            if al && !cl2 {
                continue;
            }
            if ch {
                continue;
            }
            let fans: u32 = lf_checker_rt::callee_cdecl!(CAL_FINAL, u32, pp.as_ptr() as u32);
            if (fans & 0xff) == 0 {
                return success(a0, entry);
            }
        }
    }
});
