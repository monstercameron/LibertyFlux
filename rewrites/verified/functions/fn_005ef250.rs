// original: 0x005ef250 tls_node_link

/// Link a node into the head of a thread-local list.
///
/// Stores `a` at `this+0x00`, `b` at `this+0x08` and the low byte of `flags` at
/// `this+0x0C`, then splices the node in: `this+0x04` takes the list head kept
/// at offset 4 of the thread-local block (TLS slot 0), the global list counter
/// is incremented, and the head is set to `this`. Returns `this`.
///
/// Original: thiscall, three stack arguments, no calls, callee pops 0x0C.
lf_checker_rt::export!(thiscall, rw_005ef250(this: u32, a: u32, b: u32, flags: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x04;
        const B_OFF: u32 = 0x08;
        const FLAGS: u32 = 0x0C;
        const COUNT: u32 = 0x018B7A50;
        ((this) as *mut u32).write_unaligned(a);
        ((this + B_OFF) as *mut u32).write_unaligned(b);
        ((this + FLAGS) as *mut u8).write_unaligned(flags as u8);
        let tls_block = lf_checker_rt::tls_slot(0);
        let old_head = ((tls_block + HEAD) as *const u32).read_unaligned();
        ((this + HEAD) as *mut u32).write_unaligned(old_head);
        let n = (lf_checker_rt::global::<u32>(COUNT)).read_unaligned();
        (lf_checker_rt::global::<u32>(COUNT)).write_unaligned(n.wrapping_add(1));
        ((tls_block + HEAD) as *mut u32).write_unaligned(this);
    }
    this
});
