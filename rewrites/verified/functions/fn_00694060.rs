// original: 0x00694060 comp_reinit_resolved (proposed)

//
// Reinitialises the object at `this`: stores the base vtable, resolves
// the slot at +8 through the thread-local registry (tls slot 0 -> [+4]; missing registry, a -1 lookup answer, or a null current value
// zeroes the slot, else the rebase delta is added), stores the derived
// vtable, rebases [obj+0xC] when non-null with the argument in ECX, and
// resolves the slots at +0x14/+0x18 the same way. Returns `this`. Both
// vtable immediates are relocated.
//
// Original: 0x00694060 (thiscall, one stack argument, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00694060(obj: u32, a0: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const LOOKUP: u32 = 1;
        const REBASE: u32 = 2;
        const V_BASE_FILE_VA: u32 = 0xFE38AC;
        const V_DERIVED_FILE_VA: u32 = 0xFE38DC;
        #[inline(always)]
        unsafe fn resolve_slot(slot: u32) {
            unsafe {
                let m = rd32(lf_checker_rt::tls_slot(0).wrapping_add(4));
                if m == 0 {
                    wr32(slot, 0);
                    return;
                }
                let r = lf_checker_rt::callee_thiscall!(LOOKUP, u32, rd32(m), slot);
                if r == 0xFFFF_FFFF {
                    wr32(slot, 0);
                    return;
                }
                let cur = rd32(slot);
                if cur != 0 {
                    let d = lf_checker_rt::callee_thiscall!(REBASE, u32, m, cur);
                    wr32(slot, cur.wrapping_add(d));
                }
            }
        }
        wr32(obj, lf_checker_rt::relocated(V_BASE_FILE_VA));
        resolve_slot(obj.wrapping_add(8));
        wr32(obj, lf_checker_rt::relocated(V_DERIVED_FILE_VA));
        let c = rd32(obj.wrapping_add(0x0C));
        if c != 0 {
            let d = lf_checker_rt::callee_thiscall!(REBASE, u32, a0, c);
            wr32(obj.wrapping_add(0x0C), c.wrapping_add(d));
        }
        resolve_slot(obj.wrapping_add(0x14));
        resolve_slot(obj.wrapping_add(0x18));
        obj
    }
});
