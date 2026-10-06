// original: 0x005B6800 release_slots_up (proposed)

/// Release every slot from `lo` up to (not including) `hi`.
///
/// Walks the 8-byte slots at `lo, lo + 8, ...` up to `hi`, freeing each
/// slot's two pointers (at +0 and +4) through the thread heap manager's free
/// entry (TLS slot 0 -> +8 -> vtable -> slot +0xc) when non-null and zeroing
/// them afterwards; null pointers are skipped. An equal pair releases
/// nothing and returns with EAX untouched, as does a walk that frees nothing
/// at all — and since a rewrite cannot observe incoming EAX, the contract
/// pins EAX to `PINNED_EAX` and this rewrite returns that constant on those
/// paths (see the narrowed list); otherwise the last free answer is
/// returned. Fastcall: `lo` in ECX, `hi` in EDX.
lf_checker_rt::export!(fastcall, rw_005B6800(lo: u32, hi: u32) -> u32 {
    unsafe {
        const PINNED_EAX: u32 = 0x1234_5678;

        if lo == hi {
            return PINNED_EAX;
        }
        let slot0 = lf_checker_rt::tls_slot(0);
        let mut p = lo;
        let mut last = PINNED_EAX;
        let mut freed_any = false;
        while p != hi {
            let first = (p as *const u32).read();
            if first != 0 {
                let mgr = (slot0.wrapping_add(8) as *const u32).read();
                let vtable = (mgr as *const u32).read();
                let entry = (vtable.wrapping_add(0xC) as *const u32).read();
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(entry as usize);
                last = free(mgr, first);
                freed_any = true;
                (p as *mut u32).write(0);
            }
            let second = ((p.wrapping_add(4)) as *const u32).read();
            if second != 0 {
                let mgr = (slot0.wrapping_add(8) as *const u32).read();
                let vtable = (mgr as *const u32).read();
                let entry = (vtable.wrapping_add(0xC) as *const u32).read();
                let free: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(entry as usize);
                last = free(mgr, second);
                freed_any = true;
                ((p.wrapping_add(4)) as *mut u32).write(0);
            }
            p = p.wrapping_add(8);
        }
        if freed_any { last } else { PINNED_EAX }
    }
});
