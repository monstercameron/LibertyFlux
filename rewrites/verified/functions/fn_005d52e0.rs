// original: 0x005d52e0 html_view_teardown_dispatch (proposed)

/// Tear down the two owned panes of an HTML view, then tail-dispatch.
///
/// Releases the overlay pane at `+0xe68` (member-release callee, then free
/// through the thread allocator, then clear the slot) and the document pane
/// at `+0xe60` the same way when each is non-null. Then runs the commit
/// callee on the embedded state at `+0x2b0` with `1`, and when it answers
/// non-zero (low byte compared) runs the notify callee on the same state
/// with the view. Finally tail-calls slot `+0x18` of the view-host object
/// held in the shared host global (host in ECX) and returns its answer.
///
/// Original: 0x005d52e0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_005d52e0(this: u32) -> u32 {
    unsafe {
        const OVERLAY_OFF: u32 = 0xe68;
        const DOCUMENT_OFF: u32 = 0xe60;
        const STATE_OFF: u32 = 0x2b0;
        const HOST_GLOBAL: u32 = 0x0166D9F4;
        const HOST_SLOT: u32 = 0x18;
        const OVERLAY_CALLEE: u32 = 1;
        const DOCUMENT_CALLEE: u32 = 3;
        const COMMIT_CALLEE: u32 = 4;
        const NOTIFY_CALLEE: u32 = 5;
        const FREE_OFF: u32 = 0x0c;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        #[inline(always)]
        unsafe fn release(pane: u32) {
            unsafe {
                let thread = lf_checker_rt::tls_slot(0);
                let allocator = rd32(thread + 8);
                let vtable = rd32(allocator);
                let free_fn: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable + FREE_OFF) as usize);
                free_fn(allocator, pane);
            }
        }

        let overlay = rd32(this + OVERLAY_OFF);
        if overlay != 0 {
            lf_checker_rt::callee_thiscall!(OVERLAY_CALLEE, u32, overlay);
            release(overlay);
            wr32(this + OVERLAY_OFF, 0);
        }
        let document = rd32(this + DOCUMENT_OFF);
        if document != 0 {
            lf_checker_rt::callee_thiscall!(DOCUMENT_CALLEE, u32, document);
            release(document);
            wr32(this + DOCUMENT_OFF, 0);
        }
        let committed: u32 =
            lf_checker_rt::callee_thiscall!(COMMIT_CALLEE, u32, this + STATE_OFF, 1);
        if committed & 0xff != 0 {
            lf_checker_rt::callee_thiscall!(NOTIFY_CALLEE, u32, this + STATE_OFF, this);
        }
        let host = rd32(lf_checker_rt::relocated(HOST_GLOBAL));
        let vtable = rd32(host);
        let target: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vtable + HOST_SLOT) as usize);
        target(host)
    }
});
