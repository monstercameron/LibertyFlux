// original: 0x00870580 crmt_entry_find_or_add (proposed)

/// Find the entry with the given key and refresh its synchronisation.
///
/// Waits on the mutex at `this` when non-null, then scans the 256-entry
/// table from `this` + 0xC (stride 44) for the entry whose word at +0 holds
/// `key`, comparing the index against 256 as signed. When no entry matches,
/// releases the mutex when non-null and returns the trailing import result.
/// When an entry matches and `attach` is non-zero, the entry is offered to
/// the attacher (callee 4) together with a scratch slot; then the entry's
/// semaphore at +0x1C is waited on when non-null, its counter at +0x18 is
/// bumped, the semaphore is released when non-null, the mutex at +0 is
/// released when non-null, the handle at +0x20 is waited on when non-null
/// and the semaphore at +0x24 is released with a count of one when
/// non-null. All handle tests are exact null checks. The return value is
/// the last import result, or zero when the trailing step found a null
/// handle.
///
/// The original also dead-stores the found-entry pointer over its incoming
/// key slot; no read of that slot follows on either return path, so the
/// rewrite does not reproduce it and the contract compares the stack without
/// that slot.
///
/// Original: 0x00870580 (thiscall, two stack words; the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_00870580(this: u32, key: u32, attach: u32) -> u32 {
    const WAIT_SLOT: u32 = 0xe73188;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const SEMAPHORE_SLOT: u32 = 0xe7319c;
    const ATTACH: u32 = 4;
    const INFINITE: u32 = 0xffff_ffff;
    const ENTRY_COUNT: u32 = 256;
    const ENTRY_STRIDE: u32 = 0x2c;
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
        let mutex = (this as *const u32).read_unaligned();
        if mutex != 0 {
            out = wait(mutex, INFINITE);
        }
        let mut entry: u32 = 0;
        let mut slot = this + 0x0c;
        let mut index = 0u32;
        while index < ENTRY_COUNT {
            if ((slot as *const u32).read_unaligned()) == key {
                entry = slot;
                break;
            }
            slot += ENTRY_STRIDE;
            index += 1;
        }
        if entry == 0 {
            let tail = (this as *const u32).read_unaligned();
            out = if tail != 0 { release(tail) } else { 0 };
            return out;
        }
        if (attach & 0xff) != 0 {
            let scratch: u32 = 0;
            let scratch_ptr = (&scratch as *const u32) as u32;
            lf_checker_rt::callee_thiscall!(ATTACH, u32, this + 0x2c10, scratch_ptr, entry);
        }
        let first = ((entry + 0x1c) as *const u32).read_unaligned();
        if first != 0 {
            out = wait(first, INFINITE);
        } else {
            out = 0;
        }
        let second = ((entry + 0x1c) as *const u32).read_unaligned();
        let counter = ((entry + 0x18) as *const u32).read_unaligned();
        ((entry + 0x18) as *mut u32).write_unaligned(counter.wrapping_add(1));
        if second != 0 {
            out = release(second);
        } else {
            out = 0;
        }
        let third = (this as *const u32).read_unaligned();
        if third != 0 {
            out = release(third);
        } else {
            out = 0;
        }
        let fourth = ((entry + 0x20) as *const u32).read_unaligned();
        if fourth != 0 {
            out = wait(fourth, INFINITE);
        } else {
            out = 0;
        }
        let fifth = ((entry + 0x24) as *const u32).read_unaligned();
        if fifth != 0 {
            out = semaphore(fifth, 1, 0);
        } else {
            out = 0;
        }
        out
    }
});
