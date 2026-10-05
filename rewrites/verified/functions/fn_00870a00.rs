// original: 0x00870A00 crmt_queue_push (proposed)

/// Pop a node off the free list at `this` + 0x1008, fill it and attach it.
///
/// Waits on the mutex at `this` + 0x100C when non-null. When the free list
/// anchor at +0x1008 is non-null, pops its head: the anchor takes the head's
/// next word (+4), the head's key slot (+0xC) takes the word behind the
/// `key_ptr` argument, the head's word at +0 takes `value`, and the head is
/// attached through the linker (callee 1). If the pending counter at +0x1014
/// is non-zero it is decremented, and when the semaphore at +0x1010 is then
/// also non-null it is released with a count of one. Finally releases the
/// mutex when non-null. All handle and list tests are exact null checks.
/// Returns the filled key slot address, or zero when the free list was empty.
///
/// Original: 0x00870A00 (thiscall, two stack words; the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_00870a00(this: u32, key_ptr: u32, value: u32) -> u32 {
    const LINK: u32 = 1;
    const WAIT_SLOT: u32 = 0xe73188;
    const SEMAPHORE_SLOT: u32 = 0xe7319c;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const INFINITE: u32 = 0xffff_ffff;
    unsafe {
        let wait_addr =
            (lf_checker_rt::global::<u32>(WAIT_SLOT).read_unaligned()) as usize;
        let wait: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(wait_addr);
        let semaphore_addr =
            (lf_checker_rt::global::<u32>(SEMAPHORE_SLOT).read_unaligned()) as usize;
        let semaphore: extern "stdcall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(semaphore_addr);
        let release_addr =
            (lf_checker_rt::global::<u32>(RELEASE_SLOT).read_unaligned()) as usize;
        let release: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(release_addr);
        let mut out: u32 = 0;
        let mutex = ((this + 0x100c) as *const u32).read_unaligned();
        if mutex != 0 {
            wait(mutex, INFINITE);
        }
        let head = ((this + 0x1008) as *const u32).read_unaligned();
        if head != 0 {
            let next = ((head + 4) as *const u32).read_unaligned();
            ((this + 0x1008) as *mut u32).write_unaligned(next);
            let key = (key_ptr as *const u32).read_unaligned();
            ((head + 0x0c) as *mut u32).write_unaligned(key);
            (head as *mut u32).write_unaligned(value);
            lf_checker_rt::callee_thiscall!(LINK, u32, this, head);
            out = head + 0x0c;
            let pending = ((this + 0x1014) as *const u32).read_unaligned();
            if pending != 0 {
                ((this + 0x1014) as *mut u32)
                    .write_unaligned(pending.wrapping_sub(1));
                let sem = ((this + 0x1010) as *const u32).read_unaligned();
                if sem != 0 {
                    semaphore(sem, 1, 0);
                }
            }
        }
        let tail = ((this + 0x100c) as *const u32).read_unaligned();
        if tail != 0 {
            release(tail);
        }
        out
    }
});
