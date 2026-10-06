// original: 0x005ef300 ui_elem_teardown

/// Tear down a UI element: run its two sub-object destructors, then release its
/// two optional payloads and itself through the thread-local allocator.
///
/// Callee 1 (direct) tears down the head sub-object at `this`, callee 2 (direct)
/// the tail sub-object at `this+0x20`. Then, when the flag word at `this+0x1E`
/// is nonzero and the payload at `this+0x18` is non-null, callee 3 (virtual slot 3
/// of the thread-local allocator reached through TLS slot 0) releases that payload;
/// likewise for the flag at `this+0x16` with the payload at `this+0x10`. Finally
/// callee 3 releases `this` itself. The one stack word is unused (popped by the
/// callee). Returns `this`.
///
/// Original: thiscall, one ignored stack argument, three callees, callee pops 4.
lf_checker_rt::export!(thiscall, rw_005ef300(this: u32, _unused: u32) -> u32 {
    unsafe {
        const TAIL: u32 = 0x20;
        const FLAG0: u32 = 0x1E;
        const PAY0: u32 = 0x18;
        const FLAG1: u32 = 0x16;
        const PAY1: u32 = 0x10;
        const ALLOC_OFF: u32 = 0x08;
        const RELEASE_SLOT: u32 = 0x0C;
        lf_checker_rt::callee_thiscall!(1, u32, this);
        lf_checker_rt::callee_thiscall!(2, u32, this + TAIL);
        let tls_block = lf_checker_rt::tls_slot(0);
        let alloc = ((tls_block + ALLOC_OFF) as *const u32).read_unaligned();
        let avt = ((alloc) as *const u32).read_unaligned();
        let slot = ((avt + RELEASE_SLOT) as *const u32).read_unaligned();
        let release: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let flag0 = ((this + FLAG0) as *const u16).read_unaligned();
        if flag0 != 0 {
            let pay0 = ((this + PAY0) as *const u32).read_unaligned();
            if pay0 != 0 {
                release(alloc, pay0);
            }
        }
        let flag1 = ((this + FLAG1) as *const u16).read_unaligned();
        if flag1 != 0 {
            let pay1 = ((this + PAY1) as *const u32).read_unaligned();
            if pay1 != 0 {
                release(alloc, pay1);
            }
        }
        release(alloc, this);
        this
    }
});
