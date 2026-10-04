// original: 0x00D08CC0 nearest_ped_scan_16way (proposed)
//
// Scan a fixed 16-slot ped table for entries near a center point, counting
// the matches and tracking the smallest squared distance found.
//
// Arguments (cdecl, seven stack words): `list_head` points at a holder whose
// dword at +0x224 points at the slot array, whose 16 entries start at +0x168;
// `center` points at three floats (x, y, z); `radius_bits` is the search
// radius as float bits; `filter` selects the plain path when -1, else the
// callee-filtered path; `want_word` (unless -1) must equal the sign-extended
// word at ped +0x2e; `excluded` is one ped pointer to skip; `best_out` (may
// be null) receives the running best squared distance, seeded with
// radius*radius on entry.
// Each slot is skipped when null, flagged at +0x24 bit 0x400, dead (byte at
// +0x211), of the wrong kind (dword at [+0x21c]+0x12c != 2), or at squared
// distance (point-minus-center, summed (dy^2+dx^2)+dz^2) not below
// radius*radius. The filtered path resolves each candidate through three
// intercepted callees (probe/next/test); the plain path folds it straight
// into the best. The fold compares ((dy')^2+(dx')^2)+(dz')^2 with
// center-minus-point deltas against the stored best and keeps the smaller.
// Returns the match count. Float operation order is the original's.
lf_checker_rt::export!(cdecl, rw_00D08CC0(
    list_head: u32, center: u32, radius_bits: u32, filter: u32,
    want_word: u32, excluded: u32, best_out: u32,
) -> u32 {
    unsafe {
        const SLOTS_AT: u32 = 0x224;
        const SLOTS_ENTRIES: u32 = 0x168;
        const SLOT_COUNT: u32 = 16;
        const PED_FLAGS: u32 = 0x24;
        const PED_DEAD: u32 = 0x211;
        const PED_KIND_OBJ: u32 = 0x21c;
        const PED_MTX: u32 = 0x20;
        const PED_TAG: u32 = 0x2e;
        const PED_INFO: u32 = 0x224;
        const KIND_OFF: u32 = 0x12c;
        const KIND_WANTED: u32 = 2;
        const FLAG_SKIP: u32 = 0x400;
        const MTX_X: u32 = 0x30;
        const MTX_Y: u32 = 0x34;
        const MTX_Z: u32 = 0x38;
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_NEXT: u32 = 2;
        const CALLEE_TEST: u32 = 3;
        const TEST_PROBE_ARG: u32 = 0x39b;
        const INFO_TEST_OFF: u32 = 0x2e0;
        const NONE: u32 = 0xFFFF_FFFF;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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
        /// Keep the smaller of `*best_out` and the candidate's squared
        /// distance ((dy')^2 + (dx')^2) + (dz')^2, center-minus-point.
        unsafe fn fold_best(best_out: u32, center: u32, mtx: u32) {
            unsafe {
                let best = rdf(best_out);
                let dx = sub(rdf(center), rdf(mtx.wrapping_add(MTX_X)));
                let dy = sub(rdf(center.wrapping_add(4)), rdf(mtx.wrapping_add(MTX_Y)));
                let dz = sub(rdf(center.wrapping_add(8)), rdf(mtx.wrapping_add(MTX_Z)));
                let probe = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                if probe > best {
                    return;
                }
                wrf(best_out, probe);
            }
        }

        let radius = f32::from_bits(radius_bits);
        if best_out != 0 {
            wrf(best_out, mul(radius, radius));
        }
        let table = rd32(list_head.wrapping_add(SLOTS_AT)).wrapping_add(SLOTS_ENTRIES);
        let mut count = 0u32;
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let mut ped = rd32(table.wrapping_add(i.wrapping_mul(4)));
            let mut consider = ped != 0;
            if consider && rd32(ped.wrapping_add(PED_FLAGS)) & FLAG_SKIP != 0 {
                consider = false;
            }
            if consider && rd8(ped.wrapping_add(PED_DEAD)) != 0 {
                consider = false;
            }
            if consider {
                let kind_obj = rd32(ped.wrapping_add(PED_KIND_OBJ));
                if rd32(kind_obj.wrapping_add(KIND_OFF)) != KIND_WANTED {
                    consider = false;
                }
            }
            let mut gate = false;
            let mut mtx = 0u32;
            if consider {
                mtx = rd32(ped.wrapping_add(PED_MTX));
                let dx = sub(rdf(mtx.wrapping_add(MTX_X)), rdf(center));
                let dy = sub(rdf(mtx.wrapping_add(MTX_Y)), rdf(center.wrapping_add(4)));
                let dz = sub(rdf(mtx.wrapping_add(MTX_Z)), rdf(center.wrapping_add(8)));
                let dist2 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
                gate = mul(radius, radius) > dist2;
            }
            if consider && gate {
                if ped == excluded {
                    consider = false;
                } else if want_word != NONE
                    && rd16(ped.wrapping_add(PED_TAG)) as i16 as i32 as u32 != want_word
                {
                    consider = false;
                }
            } else if consider {
                consider = false;
            }
            if !consider {
                i += 1;
                continue;
            }
            if filter == NONE {
                if best_out != 0 {
                    fold_best(best_out, center, mtx);
                }
                count = count.wrapping_add(1);
                i += 1;
                continue;
            }
            let probe: u32 = lf_checker_rt::callee_thiscall!(CALLEE_PROBE, u32, ped);
            if probe != 0 {
                let cursor = probe.wrapping_add(8);
                let n1: u32 = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                if n1 != 0 {
                    let n2: u32 = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                    if n2 != ped {
                        let info = rd32(ped.wrapping_add(PED_INFO)).wrapping_add(INFO_TEST_OFF);
                        let t: u32 = lf_checker_rt::callee_thiscall!(
                            CALLEE_TEST, u32, info, TEST_PROBE_ARG, 0
                        );
                        if (t as u8) != 0 {
                            ped = lf_checker_rt::callee_thiscall!(CALLEE_NEXT, u32, cursor);
                        }
                    }
                }
            }
            let info = rd32(ped.wrapping_add(PED_INFO)).wrapping_add(INFO_TEST_OFF);
            let t: u32 =
                lf_checker_rt::callee_thiscall!(CALLEE_TEST, u32, info, filter, 0);
            if (t as u8) != 0 {
                if best_out != 0 {
                    fold_best(best_out, center, rd32(ped.wrapping_add(PED_MTX)));
                }
                count = count.wrapping_add(1);
            }
            i += 1;
        }
        count
    }
});
