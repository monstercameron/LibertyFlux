// original: 0x00870450 crmt_batch_rebuild (proposed)

/// Rebuild the batch at `this`: drain the queue, close the handles, free the
/// array and release the tail node.
///
/// Reads the entry count from the word at `this` + 8 (compared as signed,
/// though the move-zero-extend keeps it non-negative so the sign is
/// immaterial). When positive: runs the teardown (callee 1), then once per
/// entry waits on the mutex at +0 (callee 2) when non-null, pushes a zeroed
/// frame slot through the queue (callee 3, with a zero token) and releases
/// the mutex (callee 4) when non-null; then once per entry waits on the
/// array element (callee 2) and closes it (callee 5); the array at +4 is
/// then freed through the thread-local manager (callee 7) when non-null and
/// the header words at +4 and +8 are cleared. A zero count skips all of that
/// and goes straight to the tail. When the tail node at +0x3C3C is
/// non-null it is detached (callee 6), freed through the same manager and
/// the anchor cleared. All handle tests are exact null checks. Returns the
/// final free result, or zero when no tail node was present. (When both
/// the count and the tail node are zero the original returns its incoming
/// eax untouched; the contract fixes entry eax to zero on that path.)
///
/// Original: 0x00870450 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870450(this: u32) -> u32 {
    const TEARDOWN: u32 = 1;
    const PUSH: u32 = 3;
    const DETACH: u32 = 6;
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const WAIT_SLOT: u32 = 0xe73188;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const CLOSE_SLOT: u32 = 0xe73160;
    const INFINITE: u32 = 0xffff_ffff;
    unsafe {
        let wait_addr =
            (lf_checker_rt::global::<u32>(WAIT_SLOT).read_unaligned()) as usize;
        let wait: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(wait_addr);
        let release_addr =
            (lf_checker_rt::global::<u32>(RELEASE_SLOT).read_unaligned()) as usize;
        let release: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(release_addr);
        let close_addr =
            (lf_checker_rt::global::<u32>(CLOSE_SLOT).read_unaligned()) as usize;
        let close: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(close_addr);
        let thread = lf_checker_rt::tls_slot(0);
        let manager = ((thread + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable + FREE_SLOT) as *const u32).read_unaligned();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let count = ((this + 8) as *const u16).read_unaligned() as i32;
        if count > 0 {
            lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
            let mut i = 0u32;
            while (i as i32) < count {
                let mutex = (this as *const u32).read_unaligned();
                if mutex != 0 {
                    wait(mutex, INFINITE);
                }
                let slot: u32 = 0;
                let slot_ptr = (&slot as *const u32) as u32;
                lf_checker_rt::callee_thiscall!(PUSH, u32, this + 0x2c10, slot_ptr, 0);
                let tail = (this as *const u32).read_unaligned();
                if tail != 0 {
                    release(tail);
                }
                i += 1;
            }
            let array = ((this + 4) as *const u32).read_unaligned();
            let mut j = 0u32;
            while (j as i32) < count {
                let handle = ((array + j * 4) as *const u32).read_unaligned();
                wait(handle, INFINITE);
                close(handle);
                j += 1;
            }
            let array = ((this + 4) as *const u32).read_unaligned();
            if array != 0 {
                free(manager, array);
            }
            ((this + 4) as *mut u32).write_unaligned(0);
            ((this + 8) as *mut u32).write_unaligned(0);
        }
        let node = ((this + 0x3c3c) as *const u32).read_unaligned();
        if node != 0 {
            lf_checker_rt::callee_thiscall!(DETACH, u32, node);
            let out = free(manager, node);
            ((this + 0x3c3c) as *mut u32).write_unaligned(0);
            out
        } else {
            0
        }
    }
});
