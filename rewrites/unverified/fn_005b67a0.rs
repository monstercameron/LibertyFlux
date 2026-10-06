// original: 0x005B67A0 release_slots_down (proposed)

/// Release every slot from `hi` down to (not including) `lo`.
///
/// Walks the 8-byte slots at `hi - 8, hi - 16, ...` down to `lo`, freeing
/// each slot's two pointers (at +0 and +4) through the thread heap manager's
/// free entry (TLS slot 0 -> +8 -> vtable -> slot +0xc) when non-null and
/// zeroing them afterwards; null pointers are skipped. An equal pair
/// releases nothing. The original keeps its walking pointer in its own
/// incoming stack slot (ending at `lo`); the proof switches the final-stack
/// comparison off because a rewrite cannot address that slot, and every
/// counter value takes full effect in the observed free sequence instead
/// (see the narrowed list). Returns `lo` in all cases. Cdecl: both ends on
/// the stack.
lf_checker_rt::export!(cdecl, rw_005B67A0(hi: u32, lo: u32) -> u32 {
    unsafe {
        if hi == lo {
            return hi;
        }
        let slot0 = lf_checker_rt::tls_slot(0);
        let mut p = hi;
        while p != lo {
            let s = p.wrapping_sub(8);
            let first = (s as *const u32).read();
            if first != 0 {
                let mgr = (slot0.wrapping_add(8) as *const u32).read();
                let vtable = (mgr as *const u32).read();
                let entry = (vtable.wrapping_add(0xC) as *const u32).read();
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(entry as usize);
                free(mgr, first);
                (s as *mut u32).write(0);
            }
            let second = ((s.wrapping_add(4)) as *const u32).read();
            if second != 0 {
                let mgr = (slot0.wrapping_add(8) as *const u32).read();
                let vtable = (mgr as *const u32).read();
                let entry = (vtable.wrapping_add(0xC) as *const u32).read();
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(entry as usize);
                free(mgr, second);
                ((s.wrapping_add(4)) as *mut u32).write(0);
            }
            p = p.wrapping_sub(8);
        }
        lo
    }
});
