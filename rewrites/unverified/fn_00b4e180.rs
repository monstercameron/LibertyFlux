// original: 0x00b4e180 ped_task_blend_lookup (proposed)

/// Resolve a wanted id for `this`, make a product for it, and set its blend.
///
/// `this+0x118` points to a record; a null record returns 0. The record's
/// qualifier at `+0x25c`, when non-null, must have kind `0x2e` at `+0x18`,
/// else the record itself is returned. Otherwise the record's signed 16-bit
/// index at `+0x2e` selects a pool slot from the table at `POOL`, and a
/// lookup over (slot, `G_IDLE`, `G_ACTIVE`) yields a handle whose word at
/// `+0x56` is the wanted id (`0xffff` returns `0xffff`).
///
/// The owner's chain (`this+0x78`, find-first / find-next) is scanned for an
/// entry whose id at `+0x14` matches: on a hit the alternate id (`+0x18`)
/// and blend float (`+0x4c`, as bits) come from the entry, else a resolver
/// call fills both through out-pointers. The maker then builds a product
/// from (owner, wanted, alternate, `SEL_A`, 1, `WEIGHT`); a null product
/// returns 0. A scanned entry's blend goes straight to the product's
/// setter; a resolved one returns the product itself when the low byte of
/// `flag` is clear, else the setter takes `float(counter) * FSCALE`, where
/// the counter comes from a parameterless call and `FSCALE` is the float at
/// `FCONST`. The setter's return is the result. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00b4e180(this: u32, flag: u32) -> u32 {
    unsafe {
        const REC: u32 = 0x118;
        const REC_QUAL: u32 = 0x25c;
        const QUAL_KIND: u32 = 0x18;
        const KIND_WANT: u32 = 0x2e;
        const REC_INDEX: u32 = 0x2e;
        const POOL: u32 = 0x1295CD8;
        const G_IDLE: u32 = 0x103B028;
        const G_ACTIVE: u32 = 0x103B08C;
        const H_WID: u32 = 0x56;
        const WID_NONE: u32 = 0xFFFF;
        const OWNER: u32 = 0x78;
        const E_ID: u32 = 0x14;
        const E_ALT: u32 = 0x18;
        const E_BLEND: u32 = 0x4c;
        const SEL_A: u32 = 0x204021;
        const WEIGHT: u32 = 0x447A0000;
        const FCONST: u32 = 0xFE8684;
        const LOOKUP: u32 = 1;
        const FIND_FIRST: u32 = 2;
        const FIND_NEXT: u32 = 3;
        const RESOLVE: u32 = 4;
        const MAKE: u32 = 5;
        const COUNTER: u32 = 6;
        const SET_BLEND: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let rec = rd32(this + REC);
        if rec == 0 {
            return 0;
        }
        let qual = rd32(rec + REC_QUAL);
        if qual != 0 && rd32(qual + QUAL_KIND) != KIND_WANT {
            return rec;
        }
        let index = rd16(rec + REC_INDEX) as u16 as i16 as i32 as u32;
        let slot = rd32(
            lf_checker_rt::relocated(POOL)
                .wrapping_add(index.wrapping_mul(4)),
        );
        let handle: u32 = lf_checker_rt::callee_cdecl!(
            LOOKUP, u32, slot, 0,
            lf_checker_rt::relocated(G_IDLE),
            lf_checker_rt::relocated(G_ACTIVE),
            0
        );
        if handle == 0 {
            return 0;
        }
        let wid = rd16(handle + H_WID);
        if wid == WID_NONE {
            return WID_NONE;
        }
        let owner = rd32(this + OWNER);
        let mut want_id = wid;
        let mut alt_id = 0u32;
        let mut blend = 0u32;
        let mut scanned = false;
        let mut node: u32 = lf_checker_rt::callee_thiscall!(FIND_FIRST, u32, owner);
        if node == 0 {
            lf_checker_rt::callee_cdecl!(
                RESOLVE, u32, this, 0xFFFF_FFFF,
                &mut want_id as *mut u32 as u32,
                &mut alt_id as *mut u32 as u32,
                0
            );
        } else {
            loop {
                if rd32(node + E_ID) == want_id {
                    alt_id = rd32(node + E_ALT);
                    blend = rd32(node + E_BLEND);
                    scanned = true;
                    break;
                }
                node = lf_checker_rt::callee_thiscall!(FIND_NEXT, u32, owner);
                if node == 0 {
                    lf_checker_rt::callee_cdecl!(
                        RESOLVE, u32, this, 0xFFFF_FFFF,
                        &mut want_id as *mut u32 as u32,
                        &mut alt_id as *mut u32 as u32,
                        0
                    );
                    break;
                }
            }
        }
        let product: u32 = lf_checker_rt::callee_thiscall!(
            MAKE, u32, owner, want_id, alt_id, SEL_A, 1, WEIGHT
        );
        if product == 0 {
            return 0;
        }
        let value: u32 = if scanned {
            blend
        } else {
            if flag & 0xFF == 0 {
                return product;
            }
            let n: u32 = lf_checker_rt::callee_cdecl!(COUNTER, u32);
            let scaled = mul((n as i32) as f32, f32::from_bits(rd32(lf_checker_rt::relocated(FCONST))));
            scaled.to_bits()
        };
        lf_checker_rt::callee_thiscall!(SET_BLEND, u32, product, value)
    }
});
