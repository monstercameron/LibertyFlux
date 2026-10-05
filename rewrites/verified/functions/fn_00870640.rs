// original: 0x00870640 crmt_sync_teardown (proposed)

/// Tear down the synchronisation block at `this`, waiting on and releasing
/// each live handle.
///
/// Waits on the handle at `this` when non-null. When the signed count at
/// `this` + 0x3C28 is positive, waits on the handle at +0x3C30, bumps the
/// counter at +0x3C2C, releases the handles at +0x3C30 and +0, waits on the
/// handle at +0x3C34 and releases the semaphore at +0x3C38 with a count of
/// one, each step skipped when its handle is null. When the count is zero
/// or negative only the handle at +0 is released. Every handle test is an
/// exact null check; only the count uses a signed comparison. The return
/// value is the last import result, or zero when the trailing step found a
/// null handle.
///
/// Original: 0x00870640 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870640(this: u32) -> u32 {
    const WAIT_SLOT: u32 = 0xe73188;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const SEMAPHORE_SLOT: u32 = 0xe7319c;
    const INFINITE: u32 = 0xffff_ffff;
    unsafe {
        let wait_addr =
            (lf_checker_rt::global::<u32>(WAIT_SLOT).read_unaligned()) as usize;
        let wait: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(wait_addr);
        let release_addr =
            (lf_checker_rt::global::<u32>(RELEASE_SLOT).read_unaligned()) as usize;
        let release: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(release_addr);
        let semaphore_addr =
            (lf_checker_rt::global::<u32>(SEMAPHORE_SLOT).read_unaligned()) as usize;
        let semaphore: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(semaphore_addr);
        let mut out: u32 = 0;
        let first = (this as *const u32).read_unaligned();
        if first != 0 {
            out = wait(first, INFINITE);
        } else {
            out = 0;
        }
        if ((this + 0x3c28) as *const i32).read_unaligned() > 0 {
            let second = ((this + 0x3c30) as *const u32).read_unaligned();
            if second != 0 {
                out = wait(second, INFINITE);
            } else {
                out = 0;
            }
            let third = ((this + 0x3c30) as *const u32).read_unaligned();
            let counter = ((this + 0x3c2c) as *const u32).read_unaligned();
            ((this + 0x3c2c) as *mut u32).write_unaligned(counter.wrapping_add(1));
            if third != 0 {
                out = release(third);
            } else {
                out = 0;
            }
            let fourth = (this as *const u32).read_unaligned();
            if fourth != 0 {
                out = release(fourth);
            } else {
                out = 0;
            }
            let fifth = ((this + 0x3c34) as *const u32).read_unaligned();
            if fifth != 0 {
                out = wait(fifth, INFINITE);
            } else {
                out = 0;
            }
            let sixth = ((this + 0x3c38) as *const u32).read_unaligned();
            if sixth != 0 {
                out = semaphore(sixth, 1, 0);
            } else {
                out = 0;
            }
        } else {
            let last = (this as *const u32).read_unaligned();
            if last != 0 {
                out = release(last);
            } else {
                out = 0;
            }
        }
        out
    }
});
