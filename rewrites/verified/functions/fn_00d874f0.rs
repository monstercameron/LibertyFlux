// original: 0x00d874f0 ui_tracking_update (proposed)

/// Refresh a tracked UI element from its anchor, then either reset its
/// tracking state or hand it to the range updater.
///
/// `obj` is the tracker: `+0x20` points at the anchor holding the reference
/// point (`+0x30`/`+0x34`) and the heading source (`+0x10`/`+0x14`);
/// `+0xE4C`/`+0xE50` is the tracker's own point. `elem` is the element with
/// a heading at `+0x18`, a state word pair at `+0x20`/`+0x22` and flags at
/// `+0x2B`. The distance cutoff (30.0), the measure cutoff (10.0) and the
/// length cutoff (1.5) are read from the image's read-only constants.
///
/// Algorithm: dist = length(own - anchor). If dist < 30 and the heading is
/// negative, replace it with the heading callee's answer for the anchor
/// source. If dist < 10, call the measure hook (virtual slot `+0xEC`) and,
/// when its returned vector is shorter than 1.5, clear the range index
/// (`+0xE68`) and the element state. If both state words are zero and the
/// readiness callee answers positive, clear four words at `+0x1EB4..+0x1EC0`,
/// call the reset callee and set flag bit 2; otherwise tail-call the range
/// updater with (obj, elem, 1). Comparisons use the original's unordered
/// (NaN skips the block) semantics. No value is returned.
///
/// Original: 0x00D874F0 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00d874f0(obj: u32, elem: u32) -> u32 {
    unsafe {
        const OBJ_VTABLE: u32 = 0x00;
        const OBJ_ANCHOR: u32 = 0x20;
        const OBJ_POS_X: u32 = 0x0e4c;
        const OBJ_POS_Y: u32 = 0x0e50;
        const OBJ_RANGE: u32 = 0x0e68;
        const OBJ_SLOT0: u32 = 0x1eb4;
        const OBJ_SLOT1: u32 = 0x1eb8;
        const OBJ_SLOT2: u32 = 0x1ebc;
        const OBJ_SLOT3: u32 = 0x1ec0;
        const ANCH_SRC_X: u32 = 0x10;
        const ANCH_SRC_Y: u32 = 0x14;
        const ANCH_REF_X: u32 = 0x30;
        const ANCH_REF_Y: u32 = 0x34;
        const ELEM_HEADING: u32 = 0x18;
        const ELEM_STATE: u32 = 0x20;
        const ELEM_STATE_HI: u32 = 0x22;
        const ELEM_FLAGS: u32 = 0x2b;
        const VT_MEASURE: u32 = 0xec;
        const FLAG_DONE: u8 = 0x04;
        const C_DIST_FAR: u32 = 0x00fe8b48;
        const C_DIST_NEAR: u32 = 0x00fe8b08;
        const C_LEN: u32 = 0x00fe8960;
        const CAL_HEADING: u32 = 1;
        const CAL_MEASURE: u32 = 2;
        const CAL_READY: u32 = 3;
        const CAL_RESET: u32 = 4;
        const CAL_RANGE_UPD: u32 = 5;

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
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn bb(x: f32) -> f32 {
            core::hint::black_box(x)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            bb(a) - bb(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            bb(a) * bb(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            bb(a) + bb(b)
        }
        #[inline(always)]
        fn vlen(x: f32, y: f32) -> f32 {
            bb(add(mul(x, x), mul(y, y))).sqrt()
        }

        let tail = |obj: u32, elem: u32| -> u32 {
            lf_checker_rt::callee_cdecl!(CAL_RANGE_UPD, u32, obj, elem, 1u32);
            0
        };

        let anchor = rd32(obj + OBJ_ANCHOR);
        let dx = sub(rdf(obj + OBJ_POS_X), rdf(anchor + ANCH_REF_X));
        let dy = sub(rdf(obj + OBJ_POS_Y), rdf(anchor + ANCH_REF_Y));
        let dist = vlen(dx, dy);
        if bb(rdf(lf_checker_rt::relocated(C_DIST_FAR))) > bb(dist) {
            let cur = rdf(elem + ELEM_HEADING);
            if bb(0.0) > bb(cur) {
                let h: f32 = lf_checker_rt::callee_cdecl!(
                    CAL_HEADING,
                    f32,
                    rd32(anchor + ANCH_SRC_X),
                    rd32(anchor + ANCH_SRC_Y)
                );
                wrf(elem + ELEM_HEADING, h);
            }
        }
        if bb(rdf(lf_checker_rt::relocated(C_DIST_NEAR))) > bb(dist) {
            let slot = rd32(rd32(obj + OBJ_VTABLE) + VT_MEASURE);
            let hook: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            let mut out_slot = 0u32;
            let got = hook(obj, &mut out_slot as *mut u32 as u32);
            let len = vlen(rdf(got), rdf(got + 4));
            if bb(rdf(lf_checker_rt::relocated(C_LEN))) > bb(len) {
                wr32(obj + OBJ_RANGE, 0);
                wr32(elem + ELEM_STATE, 0);
            }
        }
        if rd16(elem + ELEM_STATE) != 0 {
            return tail(obj, elem);
        }
        if rd16(elem + ELEM_STATE_HI) != 0 {
            return tail(obj, elem);
        }
        let ready: u32 = lf_checker_rt::callee_thiscall!(CAL_READY, u32, obj);
        if (ready as i32) <= 0 {
            return tail(obj, elem);
        }
        wr32(obj + OBJ_SLOT3, 0);
        wr32(obj + OBJ_SLOT0, 0);
        wr32(obj + OBJ_SLOT1, 0);
        wr32(obj + OBJ_SLOT2, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(CAL_RESET, u32, obj);
        ((elem + ELEM_FLAGS) as *mut u8).write(rd8(elem + ELEM_FLAGS) | FLAG_DONE);
        0
    }
});
