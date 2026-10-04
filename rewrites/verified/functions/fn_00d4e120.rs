// original: 0x00D4E120 CTaskSimpleAffectSecondaryBehaviour::vf17

// Update handler (vf17): polls the secondary behaviour and reports liveness.
///
/// Passes the word at `+0x18` with the fabric word at argument `+0x224`
/// (plus 0x44) to intercepted callee 1 and keeps the polled object. When the
/// byte at `+0x14` is set and the object is present and unflagged, its vtable
/// slot `+0x14` runs with (argument, 1, 0) (intercepted callee 4); a zero
/// answer yields 0, otherwise flag bit 2 is set. Control then reaches the
/// tail: vtable slot `+4` of the word at `+0x1c` runs (intercepted callee 2),
/// its answer goes with the `+0x18` word to intercepted callee 3, and the
/// result is 1. With the byte clear, a null object yields 1 at once with no
/// further calls; a present unflagged object instead runs slot `+0x14` with
/// (argument, 0, 0), setting flag bit 2 on a non-zero answer, and the result
/// is 0. (The original keeps a zero flag in its own argument slot as scratch;
/// the stack check is off for that reason.)
///
/// Original: 0x00D4E120 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e120(this: u32, arg0: u32) -> u32 {
    unsafe {
        const POLL: u32 = 1;
        const TAIL_HOOK: u32 = 2;
        const REPORT: u32 = 3;
        const APPLY: u32 = 4;
        const APPLY_SLOT: u32 = 0x14;
        const HOOK_SLOT: u32 = 4;
        const FAB_OFF: u32 = 0x224;
        const FAB_BIAS: u32 = 0x44;
        unsafe fn vcall3(obj: u32, slot: u32, a0: u32, a1: u32, a2: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let addr = ((vt + slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj, a0, a1, a2)
            }
        }
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = (obj as *const u32).read_unaligned();
                let addr = ((vt + slot) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(obj)
            }
        }
        let fab = ((arg0 + FAB_OFF) as *const u32).read_unaligned();
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        let polled: u32 =
            lf_checker_rt::callee_thiscall!(POLL, u32, fab.wrapping_add(FAB_BIAS), w18);
        if ((this + 0x14) as *const u8).read() != 0 {
            if polled != 0 && ((polled + 0xc) as *const u8).read() & 1 == 0 {
                if vcall3(polled, APPLY_SLOT, arg0, 1, 0) as u8 == 0 {
                    return 0;
                }
                let f = ((polled + 0xc) as *const u32).read_unaligned();
                ((polled + 0xc) as *mut u32).write_unaligned(f | 2);
            }
            let w1c = ((this + 0x1c) as *const u32).read_unaligned();
            let ans = vcall0(w1c, HOOK_SLOT);
            let fab2 = ((arg0 + FAB_OFF) as *const u32).read_unaligned();
            let w18b = ((this + 0x18) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(
                REPORT, u32, fab2.wrapping_add(FAB_BIAS), ans, w18b);
            return 1;
        }
        if polled == 0 {
            return 1;
        }
        if ((polled + 0xc) as *const u8).read() & 1 == 0
            && vcall3(polled, APPLY_SLOT, arg0, 0, 0) as u8 != 0
        {
            let f = ((polled + 0xc) as *const u32).read_unaligned();
            ((polled + 0xc) as *mut u32).write_unaligned(f | 2);
        }
        0
    }
});
