// original: 0x009B6C30 manager_query_store (proposed)
/// Query the secondary manager and publish the answered word.
///
/// Runs the (6, 0) query against the secondary manager, passes the caller's
/// value plus an out-word slot preset to -1 to the publish call, then stores
/// the answered word at `a1` and returns it. The original presets its own
/// saved-register slot and reuses it as the out-word; the rewrite uses a
/// private word. stdcall, value and out-pointer.
lf_checker_rt::export!(stdcall, rw_009B6C30(a0: u32, a1: u32) -> u32 {
    unsafe {
        const SEC_MGR_SLOT: u32 = 0x0103E49C;
        const PUB_MGR: u32 = 0x0103E498;
        const SLOT_INIT: u32 = 0xFFFFFFFF;
        let ga = (lf_checker_rt::global::<u32>(SEC_MGR_SLOT) as *const u32).read_unaligned();
        let r: u32 = lf_checker_rt::callee_thiscall!(1, u32, ga, 6, 0);
        let mut slot = Box::new(SLOT_INIT);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(PUB_MGR), a0, (&mut *slot as *mut u32) as u32, r);
        let v = *slot;
        (a1 as *mut u32).write_unaligned(v);
        v
    }
});
