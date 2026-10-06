// original: 0x00DF85A0 view_panel_activate (proposed)

/// Activate a view panel selected by a resolver call (thiscall, no stack
/// args, no result).
///
/// A resolver callee fills three frame words (select code plus two
/// parameters, compared SIGNED against -1: either parameter at or below
/// -1, or a zero select code, returns at once) and the select code picks
/// one of four child pointers (`+0x1E0/+0x1E4/+0x1E8/+0x1EC` for codes
/// 2/3/1/4; any other code faults on the null child, matched by the
/// rewrite). The child is initialised, then a fixed chain of vtable calls
/// walks it, its nested view and two fetched objects (slots +0x21C for a
/// parameter, +0x1E0 to fetch, +0x220/+0x218/+0x1B0/+0x18/+0x1AC to drive
/// them); a null fetch skips its block, and a null final fetch returns.
///
/// The resolver's three pointer arguments address the function's own
/// frame, so they are skipped and their preset words snapshotted instead;
/// the written select code and parameters are observed through the gates
/// they drive and the call arguments they become.
lf_checker_lf_checker_rt::export!(thiscall, rb586_fn4(this: u32) -> u32 {
    unsafe {
        const CTX: u32 = 0x019d2f18;
        const CHILD0: u32 = 0x1e0;
        const CHILD1: u32 = 0x1e4;
        const CHILD2: u32 = 0x1e8;
        const CHILD3: u32 = 0x1ec;
        const NESTED: u32 = 0x1e0;
        const S_FETCH: u32 = 0x1e0;
        const S_PARAM: u32 = 0x21c;
        const S_OPEN: u32 = 0x220;
        const S_TUNE: u32 = 0x218;
        const S_RUN: u32 = 0x1b0;
        const S_FLAG: u32 = 0x18;
        const S_APPLY: u32 = 0x1ac;
        const C_RESOLVE: u32 = 1;
        const C_INIT: u32 = 2;
        const C_PARAM: u32 = 3;
        const C_FETCH: u32 = 4;
        const C_OPEN: u32 = 5;
        const C_TUNE: u32 = 6;
        const C_RUN: u32 = 7;
        const C_FLAG: u32 = 8;
        const C_URUN: u32 = 9;
        const C_UFLAG: u32 = 10;
        const C_APPLY: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn vcall1(obj: u32, slot: u32, a0: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj) + slot) as usize);
                f(obj, a0)
            }
        }

        let mut slot_a = 0xFFFFFFFFu32;
        let mut slot_b = 0xFFFFFFFFu32;
        let mut slot_c = 0u32;
        lf_checker_rt::callee_thiscall!(
            C_RESOLVE, u32, lf_checker_rt::relocated(CTX),
            &mut slot_c as *mut u32 as u32,
            &mut slot_b as *mut u32 as u32,
            &mut slot_a as *mut u32 as u32
        );
        if slot_c == 0 {
            return 0;
        }
        if (slot_b as i32) <= -1 {
            return 0;
        }
        if (slot_a as i32) <= -1 {
            return 0;
        }
        let s = match slot_c {
            2 => rd32(this + CHILD0),
            3 => rd32(this + CHILD1),
            4 => rd32(this + CHILD3),
            1 => rd32(this + CHILD2),
            _ => 0,
        };
        lf_checker_rt::callee_thiscall!(C_INIT, u32, s, slot_b, 1);
        let t = rd32(s + NESTED);
        let q = vcall1(s, S_FETCH, vcall0(t, S_PARAM));
        if q != 0 {
            let u = vcall0(q, S_OPEN);
            vcall0(u, S_RUN);
            let u2 = vcall0(q, S_OPEN);
            vcall1(u2, S_FLAG, 0);
            vcall1(q, S_TUNE, 0);
        }
        let q2 = vcall1(s, S_FETCH, vcall0(t, S_PARAM));
        vcall0(q2, S_RUN);
        let q3 = vcall1(s, S_FETCH, vcall0(t, S_PARAM));
        vcall1(q3, S_FLAG, 0);
        let r = vcall1(s, S_FETCH, slot_b);
        if r == 0 {
            return 0;
        }
        vcall1(r, S_TUNE, slot_a);
        let w = vcall0(r, S_OPEN);
        vcall0(w, S_APPLY);
        let w2 = vcall0(r, S_OPEN);
        vcall1(w2, S_FLAG, 1);
        0
    }
});
