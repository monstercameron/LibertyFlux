// original: 0x00b0ba70 net_track_nearest_slot (proposed)

/// Consider the calling object's position for the nearest-slot table.
///
/// `obj` points at the caller's record. When its generation word (`+0x2e`)
/// matches the current generation and its three-deep handle chain is alive,
/// the record's status callback (virtual slot `+0xa0`) runs; a null answer
/// falls back to the record's stored handle (`+0x100`), otherwise the
/// callback runs again and its result's detail callback (slot `+0xe0`) runs.
/// A null outcome there, or a null stored handle, runs the record's reset
/// helper instead.
///
/// Then the position is read (from the linked block at `+0x20`, bias `+0x30`,
/// or from `+0x10` when no block is linked), raised by the height bias, and
/// its distance to the reference point is taken. Distances at or above the
/// radius are ignored. Otherwise, when fewer than twelve slots are filled
/// the position takes the next one; when the table is full it replaces the
/// currently farthest slot, but only when some slot lies farther than the
/// new position. Each stored slot keeps the distance, the capped strength
/// factor, the raised position and the generation word. The fourth stored
/// component comes from a stack slot the original never writes; the contract
/// fills uninitialized stack with zero, so it reads `0.0`.
///
/// Original: 0x00b0ba70 (cdecl, one stack word).
export!(cdecl, rw_00b0ba70(obj: u32) -> u32 {
    unsafe {
        const GEN_WORD: u32 = 0x2e;
        const CHAIN_PTR: u32 = 0x34;
        const CHAIN_FLAG: u32 = 0x0c;
        const VT_STATUS: u32 = 0xa0;
        const VT_DETAIL: u32 = 0xe0;
        const STORED_HANDLE: u32 = 0x100;
        const LINK: u32 = 0x20;
        const LINK_POS: u32 = 0x30;
        const OWN_POS: u32 = 0x10;
        const MAX_SLOTS: i32 = 12;
        const TABLE_DIST: u32 = 0x16154e0;
        const TABLE_STR: u32 = 0x1615510;
        const TABLE_GEN: u32 = 0x1615540;
        const TABLE_POS: u32 = 0x1632b20;
        const COUNT: u32 = 0x1615570;
        const GENERATION: u32 = 0x12fa3e0;
        const REF_X: u32 = 0x128e340;
        const REF_Y: u32 = 0x128e344;
        const REF_Z: u32 = 0x128e348;
        const HEIGHT_BIAS: u32 = 0xfe87d0;
        const RADIUS: u32 = 0xfe8b48;
        const FULL_STR: u32 = 0xfe88e8;
        const STR_BASE: u32 = 0xeaabc0;
        const STR_SCALE: u32 = 0xfe87a4;
        const C_STATUS: u32 = 1;
        const C_DETAIL: u32 = 2;
        const C_RESET: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn gf(va: u32) -> f32 {
            f32::from_bits(g32(va))
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

        let gen = rd16(obj + GEN_WORD) as i16 as i32;
        if gen == g32(GENERATION) as i32 {
            let h1 = rd32(obj + CHAIN_PTR);
            if h1 != 0 {
                let h2 = rd32(h1);
                if h2 != 0 && rd32(h2 + CHAIN_FLAG) != 0 {
                    let vt = rd32(obj);
                    let status: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(rd32(vt + VT_STATUS) as usize);
                    let r1 = status(obj);
                    let out = if r1 == 0 {
                        rd32(obj + STORED_HANDLE)
                    } else {
                        let r2 = status(obj);
                        let vt2 = rd32(r2);
                        let detail: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(vt2 + VT_DETAIL) as usize);
                        detail(r2)
                    };
                    if out == 0 {
                        let _: u32 = lf_checker_rt::callee_thiscall!(C_RESET, u32, obj);
                    }
                }
            }
        }

        let link = rd32(obj + LINK);
        let base = if link != 0 { link.wrapping_add(LINK_POS) } else { obj + OWN_POS };
        let px = rdf(base);
        let py = rdf(base + 4);
        let pz = add(rdf(base + 8), gf(HEIGHT_BIAS));
        let dx = sub(px, gf(REF_X));
        let dy = sub(py, gf(REF_Y));
        let dz = sub(pz, gf(REF_Z));
        let sumsq = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let dist = core::hint::black_box(sumsq).sqrt();
        if !(dist < gf(RADIUS)) {
            return 0;
        }
        let raw = sub(gf(FULL_STR), mul(sub(dist, gf(STR_BASE)), gf(STR_SCALE)));
        let strength = if gf(FULL_STR) > raw { raw } else { gf(FULL_STR) };
        let countp = lf_checker_rt::global::<u32>(COUNT) as *mut u32;
        let count = countp.read_unaligned() as i32;
        let slot: i32 = if count < MAX_SLOTS {
            countp.write_unaligned(count.wrapping_add(1) as u32);
            count
        } else {
            let mut best = dist;
            let mut idx: i32 = -1;
            for k in 0..12u32 {
                let e = f32::from_bits(g32(TABLE_DIST.wrapping_add(k * 4)));
                if e > best {
                    best = e;
                    idx = k as i32;
                }
            }
            idx
        };
        if slot < 0 {
            return 0;
        }
        let s4 = (slot as u32).wrapping_mul(4);
        let s16 = (slot as u32).wrapping_mul(16);
        wrf(lf_checker_rt::relocated(TABLE_DIST) + s4, dist);
        wrf(lf_checker_rt::relocated(TABLE_STR) + s4, strength);
        wrf(lf_checker_rt::relocated(TABLE_POS) + s16, px);
        wrf(lf_checker_rt::relocated(TABLE_POS) + s16 + 4, py);
        wrf(lf_checker_rt::relocated(TABLE_POS) + s16 + 8, pz);
        wrf(lf_checker_rt::relocated(TABLE_POS) + s16 + 12, 0.0);
        wr32(lf_checker_rt::relocated(TABLE_GEN) + s4, gen as u32);
        0
    }
});
