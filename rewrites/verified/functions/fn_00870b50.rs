// original: 0x00870B50 crmt_list_find_and_attach (proposed)

/// Find the node whose key slot matches and attach it, reporting success.
///
/// Waits on the mutex at `this` + 0x100C when non-null, then walks the
/// list anchored at +0x1000 (next at node +4) comparing each node's key
/// slot at +0xC against the word behind the `key_ptr` argument. On a match
/// whose flag word at +0 is not yet set, detaches the node through the first
/// helper (callee 1), marks the flag all-ones and attaches it through the
/// second helper (callee 2). A match that is already marked skips both
/// helpers. Releases the mutex when non-null. Both list comparisons are
/// exact-equality tests. Returns 1 on a match and 0 on a miss, carried in
/// the low byte over the previous import result.
///
/// The second stack argument is received but never read.
///
/// Original: 0x00870B50 (thiscall, two stack words; the callee pops 8 bytes).
lf_checker_rt::export!(thiscall, rw_00870b50(this: u32, key_ptr: u32, _unused: u32) -> u32 {
    const DETACH: u32 = 1;
    const ATTACH: u32 = 2;
    const WAIT_SLOT: u32 = 0xe73188;
    const RELEASE_SLOT: u32 = 0xe731b0;
    const INFINITE: u32 = 0xffff_ffff;
    const LOW_MASK: u32 = 0xffff_ff00;
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
        let key = (key_ptr as *const u32).read_unaligned();
        let mut node = ((this + 0x1000) as *const u32).read_unaligned();
        let mut found = false;
        while node != 0 {
            if ((node + 0x0c) as *const u32).read_unaligned() == key {
                found = true;
                break;
            }
            node = ((node + 4) as *const u32).read_unaligned();
        }
        if found {
            if ((node as *const u32).read_unaligned()) != 0xffff_ffff {
                lf_checker_rt::callee_thiscall!(DETACH, u32, this, node);
                (node as *mut u32).write_unaligned(0xffff_ffff);
                lf_checker_rt::callee_thiscall!(ATTACH, u32, this, node);
            }
            let tail = ((this + 0x100c) as *const u32).read_unaligned();
            if tail != 0 {
                release(tail) & LOW_MASK | 1
            } else {
                1
            }
        } else {
            let tail = ((this + 0x100c) as *const u32).read_unaligned();
            if tail != 0 {
                release(tail) & LOW_MASK
            } else {
                0
            }
        }
    }
});
