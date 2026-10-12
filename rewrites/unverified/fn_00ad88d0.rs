// original: 0x00AD88D0 ui_queue_flush

/// Flushes a queued UI batch when the enable byte is nonzero. A signed
/// positive pending count causes four cdecl service calls, in order, using
/// the queued resource, its payload at offset `+0x0C`, and the pending count.
/// The pending count is cleared after an enabled flush even when it is
/// nonpositive; a disabled flush leaves all state untouched.
lf_checker_rt::export!(cdecl, rw_00ad88d0() -> u32 {
    const ENABLE_BYTE: u32 = 0x0154E2B5;
    const QUEUED_RESOURCE: u32 = 0x01550EA0;
    const QUEUE_CONTEXT: u32 = 0x01550EA4;
    const PENDING_COUNT: u32 = 0x01550EA8;

    if unsafe { lf_checker_rt::global::<u8>(ENABLE_BYTE).read_volatile() } == 0 {
        return 0;
    }

    let pending_count = unsafe { lf_checker_rt::global::<i32>(PENDING_COUNT).read_volatile() };
    if pending_count > 0 {
        const RELEASE_RESOURCE: u32 = 1;
        const SUBMIT_BATCH: u32 = 2;
        const SET_BATCH_LIMIT: u32 = 3;
        const END_BATCH: u32 = 4;

        let resource = unsafe { lf_checker_rt::global::<u32>(QUEUED_RESOURCE).read_volatile() };
        let context = unsafe { lf_checker_rt::global::<u32>(QUEUE_CONTEXT).read_volatile() };
        let payload = unsafe { (context.wrapping_add(0x0C) as *const u32).read_unaligned() };

        let _ = lf_checker_rt::callee_cdecl!(RELEASE_RESOURCE, u32, resource);
        let _ = lf_checker_rt::callee_cdecl!(SUBMIT_BATCH, u32, 0u32, context, 0u32, payload);
        let _ = lf_checker_rt::callee_cdecl!(SET_BATCH_LIMIT, u32, 4u32, 0u32, pending_count as u32);
        let _ = lf_checker_rt::callee_cdecl!(END_BATCH, u32, 0u32);
    }

    unsafe { lf_checker_rt::global::<u32>(PENDING_COUNT).write_volatile(0) };
    0
});
