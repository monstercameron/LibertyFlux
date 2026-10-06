// original: 0x00693F80 comp_ctor_resolved (proposed)

//
// Constructs the object at `this`: stores the vtable and scalar fields,
// resolves the two pointer slots at +0x14/+0x18 through the thread-local
// registry each (tls slot 0 -> [+4]; missing registry, a -1 answer
// from the intercepted lookup, or a null current value zeroes the slot,
// else the intercepted rebase delta is added), clears +0x1C/+0x28/+0x2C,
// stores the 0x3C23D70A/0x64 pair at +0x20/+0x24, and runs the intercepted
// three-argument initialiser with `this` in ECX. Returns `this`. The
// vtable immediate is relocated.
//
// Original: 0x00693F80 (thiscall, three stack arguments, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693F80(obj: u32, a0: u32, a1: u32, a2: u32) -> u32 {
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
        const INIT: u32 = 3;
        const V_FILE_VA: u32 = 0xFE38DC;
        const MAGIC_F: u32 = 0x3C23D70A;
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
        wr32(obj.wrapping_add(8), 0);
        wr32(obj.wrapping_add(4), 3);
        wr32(obj, lf_checker_rt::relocated(V_FILE_VA));
        wr32(obj.wrapping_add(0x10), 0);
        wr32(obj.wrapping_add(0x0C), 0);
        resolve_slot(obj.wrapping_add(0x14));
        resolve_slot(obj.wrapping_add(0x18));
        wr32(obj.wrapping_add(0x1C), 0);
        wr32(obj.wrapping_add(0x20), MAGIC_F);
        wr32(obj.wrapping_add(0x24), 0x64);
        wr32(obj.wrapping_add(0x28), 0);
        wr32(obj.wrapping_add(0x2C), 0);
        lf_checker_rt::callee_thiscall!(INIT, u32, obj, a0, a1, a2);
        obj
    }
});
