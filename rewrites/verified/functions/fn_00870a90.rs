// original: 0x00870A90 crmt_spin_then_unlink (proposed)

/// Unlink the head node of the live list at `this` and push it on the free list.
///
/// Waits on the mutex at `this` + 0x100C when non-null, then spins until the
/// live-list anchor at +0x1000 is non-null (each spin pass bumps the counter
/// at +0x1014 and re-waits the handles; with stubbed callees this loop can
/// only exit when the anchor is already set, so the contract pins it
/// non-null). The head node is then unlinked: the anchor at +0x1004 takes the
/// head's next word (+8), and either the next node's back-link (+4) is
/// cleared or, when there is no next node, the live anchor at +0x1000 is.
/// The head's word at +4 takes the free-list anchor at +0x1008 and the head
/// itself becomes the new free-list anchor. Finally releases the mutex when
/// non-null. All handle tests are exact null checks. Returns the head's word
/// at +0xC.
///
/// Original: 0x00870A90 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00870a90(this: u32) -> u32 {
    const WAIT_SLOT: u32 = 0xe73188;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const INFINITE: u32 = 0xffff_ffff;
    unsafe {
        let wait_addr =
            (lf_checker_rt::global::<u32>(WAIT_SLOT).read_unaligned()) as usize;
        let wait: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(wait_addr);
        let release_addr =
            (lf_checker_rt::global::<u32>(RELEASE_SLOT).read_unaligned()) as usize;
        let release: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(release_addr);
        let mutex = ((this + 0x100c) as *const u32).read_unaligned();
        if mutex != 0 {
            wait(mutex, INFINITE);
        }
        while ((this + 0x1000) as *const u32).read_unaligned() == 0 {
            let counter = ((this + 0x1014) as *const u32).read_unaligned();
            ((this + 0x1014) as *mut u32).write_unaligned(counter.wrapping_add(1));
            let live = ((this + 0x100c) as *const u32).read_unaligned();
            if live != 0 {
                release(live);
            }
            let sem = ((this + 0x1010) as *const u32).read_unaligned();
            if sem != 0 {
                wait(sem, INFINITE);
            }
            let again = ((this + 0x100c) as *const u32).read_unaligned();
            if again != 0 {
                wait(again, INFINITE);
            }
        }
        let head = ((this + 0x1004) as *const u32).read_unaligned();
        let next = ((head + 8) as *const u32).read_unaligned();
        let out = ((head + 0x0c) as *const u32).read_unaligned();
        ((this + 0x1004) as *mut u32).write_unaligned(next);
        if next != 0 {
            ((next + 4) as *mut u32).write_unaligned(0);
        } else {
            ((this + 0x1000) as *mut u32).write_unaligned(0);
        }
        let free = ((this + 0x1008) as *const u32).read_unaligned();
        ((head + 4) as *mut u32).write_unaligned(free);
        ((this + 0x1008) as *mut u32).write_unaligned(head);
        let tail = ((this + 0x100c) as *const u32).read_unaligned();
        if tail != 0 {
            release(tail);
        }
        out
    }
});
