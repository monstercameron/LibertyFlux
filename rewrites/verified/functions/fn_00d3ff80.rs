// original: 0x00d3ff80 jump_probe_land_clearance (proposed)

/// Probe nearby cover/tasks to decide whether a jump-landing is clear.
///
/// `owner` (arg0) links the probe object at `PROBE_OBJ` (+0x78). One
/// probe callee (thiscall on that object, one id word) is queried up to
/// nine times with fixed ids (0x32, then 0x28-0x2b, then 0x1c-0x1f),
/// stopping a group early on the first non-null answer. A non-null answer
/// points to a record whose float at `REC_DIST` (+0x58) is compared
/// against 0.3 (ordered; unordered counts as not-above): above it keeps
/// probing, at-or-below it moves on. After the groups, a surviving record
/// whose distance is strictly above 0.3 yields `REC_BIAS` (+0x4c) + 0.367,
/// minus 1.0 when strictly above 1.0; otherwise the result is 0.0. The
/// function returns 1 when 0.5 is strictly above that result (ordered),
/// else 0. Only al carries the result.
///
/// No writes, no globals; the four threshold constants are image reads.
///
/// Original: 0x00d3ff80 (stdcall, one stack word; incoming ecx unused).
lf_checker_rt::export!(stdcall, rw_00d3ff80(owner: u32) -> u32 {
    unsafe {
        const PROBE_OBJ: u32 = 0x78;
        const PROBE: u32 = 1;
        const REC_BIAS: u32 = 0x4c;
        const REC_DIST: u32 = 0x58;
        const C_NEAR: u32 = 0x00fe_87e8;
        const C_BIAS: u32 = 0x00ee_3ee0;
        const C_ONE: u32 = 0x00fe_88e8;
        const C_HALF: u32 = 0x00fe_8830;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn fconst(file_va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(file_va))) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let obj = rd32(owner + PROBE_OBJ);
        let near = fconst(C_NEAR);
        // First probe: a hit within range moves on, else the 0x28 group runs.
        let mut hit: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, obj, 0x32);
        if hit == 0 || near > rdf(hit + REC_DIST) {
            for id in [0x28u32, 0x29, 0x2a, 0x2b] {
                hit = lf_checker_rt::callee_thiscall!(PROBE, u32, obj, id);
                if hit != 0 {
                    break;
                }
            }
        }
        // Middle gate: a hit within range moves on, else the 0x1c group runs.
        if hit == 0 || near > rdf(hit + REC_DIST) {
            for id in [0x1cu32, 0x1d, 0x1e, 0x1f] {
                hit = lf_checker_rt::callee_thiscall!(PROBE, u32, obj, id);
                if hit != 0 {
                    break;
                }
            }
        }
        let mut out = 0.0f32;
        if hit != 0 && rdf(hit + REC_DIST) > near {
            let grown = add(rdf(hit + REC_BIAS), fconst(C_BIAS));
            if grown > fconst(C_ONE) {
                out = sub(grown, fconst(C_ONE));
            } else {
                out = grown;
            }
        }
        if fconst(C_HALF) > out { 1 } else { 0 }
    }
});
