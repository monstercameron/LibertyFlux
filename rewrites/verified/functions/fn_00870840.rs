// original: 0x00870840 crmt_node_attach (proposed)

/// Attach the node at `this` + 0x2C0C to the queue, stamping the entry float.
///
/// Carries the caller's float from vector register 2 (low word only; the
/// upper words never reach memory). Waits on the mutex at `this` when
/// non-null. When the pending-node word at +0x2C0C is non-null, unlinks it
/// (the anchor takes the node's next word at +0x28), stamps the node:
/// arg0 at +0, the entry float bit-exact at +4, the two words behind `src`
/// at +8 and +0xC, the two words behind `dst` at +0x10 and +0x14, then
/// pushes the node through the queue (callee 1, receiving a frame slot
/// holding the node address alongside arg1), bumps the counter at +0x3C28
/// and releases the mutex when non-null. When no node is pending only the
/// mutex is released. All handle tests are exact null checks. Returns 1
/// when a node was attached and 0 otherwise, carried in the low byte over
/// the previous import result.
///
/// Original: 0x00870840 (thiscall, four stack words; the callee pops 16 bytes).
lf_checker_rt::export!(thiscall, rw_00870840(this: u32, tag: u32, token: u32, src: u32, dst: u32) -> u32 {
    const PUSH: u32 = 1;
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
        let weight = lf_checker_rt::xmm_word(2, 0);
        let mutex = (this as *const u32).read_unaligned();
        if mutex != 0 {
            wait(mutex, INFINITE);
        }
        let node = ((this + 0x2c0c) as *const u32).read_unaligned();
        if node == 0 {
            let tail = (this as *const u32).read_unaligned();
            if tail != 0 {
                release(tail) & LOW_MASK
            } else {
                0
            }
        } else {
            let next = ((node + 0x28) as *const u32).read_unaligned();
            ((this + 0x2c0c) as *mut u32).write_unaligned(next);
            (node as *mut u32).write_unaligned(tag);
            ((node + 4) as *mut u32).write_unaligned(weight);
            ((node + 8) as *mut u32).write_unaligned((src as *const u32).read_unaligned());
            ((node + 0x0c) as *mut u32)
                .write_unaligned(((src + 4) as *const u32).read_unaligned());
            ((node + 0x10) as *mut u32).write_unaligned((dst as *const u32).read_unaligned());
            ((node + 0x14) as *mut u32)
                .write_unaligned(((dst + 4) as *const u32).read_unaligned());
            let mut slot: u32 = node;
            let slot_ptr: u32 = (&mut slot as *mut u32) as u32;
            lf_checker_rt::callee_thiscall!(PUSH, u32, this + 0x2c10, slot_ptr, token);
            let counter = ((this + 0x3c28) as *const u32).read_unaligned();
            ((this + 0x3c28) as *mut u32).write_unaligned(counter.wrapping_add(1));
            let tail = (this as *const u32).read_unaligned();
            if tail != 0 {
                release(tail) & LOW_MASK | 1
            } else {
                1
            }
        }
    }
});
