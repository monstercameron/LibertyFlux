// original: 0x00a95980 stream_quota_from_thread_state (proposed)

/// Cap the argument by the calling thread's streaming quota.
///
/// The thread-state block for this module (TLS slot from its global) leads
/// to a provider object (slot `+8`, dereferenced twice); the provider's
/// slot-0x10 virtual (callee 1, thiscall/1 with 1) returns a limiter whose
/// slot-0x1c virtual (callee 2, thiscall/1 with -1) and slot-0x20 virtual
/// (callee 3, thiscall/0) report two quota parts. The argument is capped
/// above by their sum (unsigned) into `this+0x20`.
///
/// Returns the second quota part. Thiscall: object in ecx, the cap on the
/// stack, callee pops 4.
lf_checker_rt::export!(thiscall, rw_00a95980(this: u32, cap: u32) -> u32 {
    unsafe {
        const SLOT_GLOBAL: u32 = 0x017aba14;
        const PROVIDER_OFF: u32 = 8;
        const CAP_OFF: u32 = 0x20;
        let idx = lf_checker_rt::global::<u32>(SLOT_GLOBAL).read_unaligned();
        let blk = lf_checker_rt::tls_slot(idx as usize) as u32;
        let provider = ((blk + PROVIDER_OFF) as *const u32).read_unaligned();
        let vt = (provider as *const u32).read_unaligned();
        let t1 = ((vt + 0x10) as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(t1 as usize);
        let limiter = f1(provider, 1);
        let w = (limiter as *const u32).read_unaligned();
        let t2 = ((w + 0x1c) as *const u32).read_unaligned();
        let f2: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(t2 as usize);
        let q1 = f2(limiter, 0xffffffff);
        let t3 = ((w + 0x20) as *const u32).read_unaligned();
        let f3: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(t3 as usize);
        let q2 = f3(limiter);
        let total = q1.wrapping_add(q2);
        ((this + CAP_OFF) as *mut u32)
            .write_unaligned(if total < cap { total } else { cap });
        q2
    }
});
