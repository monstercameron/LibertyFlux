// original: 0x00B3A300 ped_task_slot_validate (proposed)

/// Validate a task slot through a chain of gates, returning 1 only if every
/// gate passes and 0 at the first failure.
///
/// `center` points at three floats (x, y, z). Flag bytes select gates:
/// `flag_c` (resolve gate), `flag_14` (resolve-verify skip), `flag_18`
/// (density gate), `flag_1c` (range gate), `flag_24` (tail gate). `radius`
/// is the exclusion radius, `f20` a float forwarded to the range callee,
/// `arg28` an opaque word, `tab_idx` the tail-table index, `arg30` an id skipped by the scan,
/// `out_count` receives the resolve count, and `arg38` an otherwise unread word observed only through the resolve-gate snapshot.
///
/// Behaviour: gate 1 calls the pre-check callee with `center`; a nonzero
/// low byte fails. The range gate calls its callee with (`center`, `radius`,
/// `f20`); nonzero fails. The resolve gate calls its callee with (`center`,
/// an out-pair slot, 2.0): a null return fails, as does a count at or below
/// zero; otherwise the count is stored to `out_count` and the return value
/// is stored through the out-pointer. The verify callee is then probed and,
/// unless skipped, a confirm callee with (stored value, count); a zero
/// return fails. Two pool scans follow: each pool global points at
/// (base, flag-bytes, count, stride); entries run from the last to the
/// first, skipping flag bytes with 0x80 set (and, in the second pool,
/// entries with a nonzero marker byte) and null entries. Each live entry's
/// position (through its word at +0x20, floats at +0x30) gives a squared
/// distance `((dy^2+dx^2)+dz^2)` in that operation order; a distance below
/// the squared radius fails, one below the squared threshold global counts
/// a neighbour. The density gate fails when the neighbour total exceeds its
/// limit global (signed). A region callee is then probed with (`center`,
/// `radius`, 1, 2, 0, 1, 1, `arg28`); a zero low byte runs a ten-slot scan
/// over the function's own zero stores, so every slot is skipped and the two
/// probe callees never fire (dead code in the original as well: no callee
/// writes those slots). The tail gate calls its callee; a
/// nonzero answer runs a final probe callee with (`center`, the table
/// entry's float at +0x1c, `G_TAIL_A * G_TAIL_B`, `G_TAIL_C`, 0, 0) whose
/// nonzero answer fails. Every return path invokes the security-cookie
/// callee first, like the original.
///
/// Original: 0x00B3A300 (cdecl, thirteen stack words, returns 0/1 in al).
lf_checker_rt::export!(cdecl, rw_00B3A300(center: u32, flag_c: u32, radius: u32, flag_14: u32, flag_18: u32, flag_1c: u32, f20: u32, flag_24: u32, arg28: u32, tab_idx: u32, arg30: u32, out_count: u32, arg38: u32) -> u32 {
    unsafe {
        const PRE_CALLEE: u32 = 1;
        const RANGE_CALLEE: u32 = 2;
        const RESOLVE_CALLEE: u32 = 3;
        const VERIFY_CALLEE: u32 = 4;
        const CONFIRM_CALLEE: u32 = 5;
        const REGION_CALLEE: u32 = 6;
        const TAIL_CALLEE: u32 = 9;
        const FINAL_CALLEE: u32 = 10;
        const COOKIE_CALLEE: u32 = 11;
        const POOL_A: u32 = 0x18B6F10;
        const POOL_B: u32 = 0x18B6F1C;
        const G_THRESH: u32 = 0x1045954;
        const G_LIMIT: u32 = 0x1045950;
        const G_TAIL_C: u32 = 0x104597C;
        const G_TAIL_A: u32 = 0x103F6BC;
        const G_TAIL_B: u32 = 0x1045980;
        const TAIL_TABLE: u32 = 0x1295CD8;
        const FINAL_THIS: u32 = 0x1908EF0;
        const TWO_BITS: u32 = 0x4000_0000;

        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn cookie() {
            lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        }

        let r1: u32 = lf_checker_rt::callee_cdecl!(PRE_CALLEE, u32, center);
        if (r1 as u8) != 0 {
            cookie();
            return 0;
        }
        if (flag_1c as u8) != 0 {
            let r2: u32 =
                lf_checker_rt::callee_cdecl!(RANGE_CALLEE, u32, center, radius, f20);
            if (r2 as u8) != 0 {
                cookie();
                return 0;
            }
        }
        if (flag_c as u8) != 0 {
            let mut out_pair = [0xFFFF_FFFFu32, arg38];
            let r3: u32 = lf_checker_rt::callee_cdecl!(
                RESOLVE_CALLEE,
                u32,
                center,
                out_pair.as_mut_ptr() as u32,
                TWO_BITS
            );
            if r3 == 0 {
                cookie();
                return 0;
            }
            let count = out_pair[0] as i32;
            if count <= 0 {
                cookie();
                return 0;
            }
            (out_count as *mut u32).write(count as u32);
            let p = out_pair[1];
            (p as *mut u32).write(r3);
            let r4: u32 = lf_checker_rt::callee_thiscall!(VERIFY_CALLEE, u32, r3);
            if (flag_14 as u8) == 0 && (r4 as u8) != 0 {
                let stored = (p as *const u32).read();
                let r5: u32 =
                    lf_checker_rt::callee_thiscall!(CONFIRM_CALLEE, u32, stored, count as u32);
                if r5 == 0 {
                    cookie();
                    return 0;
                }
            }
        }
        let rad = f32::from_bits(radius);
        let rad2 = fmul(rad, rad);
        let thr = rdf(lf_checker_rt::relocated(G_THRESH));
        let thr2 = fmul(thr, thr);
        let mut near = 0u32;
        for pass in 0..2u32 {
            let g = if pass == 0 { POOL_A } else { POOL_B };
            let pool = lf_checker_rt::global::<u32>(g).read();
            let base = rd32(pool);
            let flags = rd32(pool.wrapping_add(4));
            let n = rd32(pool.wrapping_add(8));
            let stride = rd32(pool.wrapping_add(12));
            if n != 0 {
                let mut ecx = n.wrapping_sub(1);
                loop {
                    let fb: u8 = ((flags.wrapping_add(ecx)) as *const u8).read();
                    if fb & 0x80 == 0 {
                        let entry = base.wrapping_add(stride.wrapping_mul(ecx));
                        let mut live = entry != 0;
                        if live && pass == 1 {
                            let mark: u8 =
                                ((entry.wrapping_add(0x219)) as *const u8).read();
                            if mark != 0 {
                                live = false;
                            }
                        }
                        if live {
                            let pos = rd32(entry.wrapping_add(0x20));
                            let dx = fsub(rdf(center), rdf(pos.wrapping_add(0x30)));
                            let dy = fsub(rdf(center.wrapping_add(4)), rdf(pos.wrapping_add(0x34)));
                            let dz = fsub(rdf(center.wrapping_add(8)), rdf(pos.wrapping_add(0x38)));
                            let d2 = fadd(fadd(fmul(dy, dy), fmul(dx, dx)), fmul(dz, dz));
                            if rad2 > d2 {
                                cookie();
                                return 0;
                            }
                            if thr2 > d2 {
                                near = near.wrapping_add(1);
                            }
                        }
                    }
                    if ecx == 0 {
                        break;
                    }
                    ecx = ecx.wrapping_sub(1);
                }
            }
        }
        if (flag_18 as u8) != 0 {
            let limit = lf_checker_rt::global::<u32>(G_LIMIT).read() as i32;
            if (near as i32) > limit {
                cookie();
                return 0;
            }
        }
        let r6: u32 = lf_checker_rt::callee_cdecl!(
            REGION_CALLEE, u32, center, radius, 1u32, 2u32, 0u32, 1u32, 1u32, arg28
        );
        if (r6 as u8) == 0 {
            // The ten scan slots are the function's own zero stores, so every
            // slot is skipped and the probe callees never fire (also true of
            // the original: the scan is dead code there as well).
            let slots = [0u32; 10];
            let mut counter = 0u32;
            for i in 0..10u32 {
                let v = slots[i as usize];
                if v == 0 {
                    continue;
                }
                if v == arg30 {
                    continue;
                }
            }
            if (counter as i32) > 0 {
                cookie();
                return 0;
            }
        }
        if (flag_24 as u8) != 0 {
            let r9: u32 = lf_checker_rt::callee_cdecl!(TAIL_CALLEE, u32,);
            if (r9 as u8) != 0 {
                let ent = rd32(lf_checker_rt::relocated(TAIL_TABLE)
                    .wrapping_add(tab_idx.wrapping_mul(4)));
                let f1 = rdf(ent.wrapping_add(0x1c));
                let f0 = fmul(
                    rdf(lf_checker_rt::relocated(G_TAIL_A)),
                    rdf(lf_checker_rt::relocated(G_TAIL_B)),
                );
                let fc = rdf(lf_checker_rt::relocated(G_TAIL_C));
                let r10: u32 = lf_checker_rt::callee_thiscall!(
                    FINAL_CALLEE,
                    u32,
                    lf_checker_rt::relocated(FINAL_THIS),
                    center,
                    f1.to_bits(),
                    f0.to_bits(),
                    fc.to_bits(),
                    0u32,
                    0u32
                );
                if (r10 as u8) != 0 {
                    cookie();
                    return 0;
                }
            }
        }
        cookie();
        1
    }
});

///
