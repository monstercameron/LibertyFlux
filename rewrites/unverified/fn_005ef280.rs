// original: 0x005ef280 tls_node_unlink

/// Unlink a node from the head of a thread-local list.
///
/// Writes the node's link word (`this+0x04`) back over the list head kept at
/// offset 4 of the thread-local block (TLS slot 0) and decrements the global
/// list counter. Returns the restored head (the node's old link word).
///
/// Original: thiscall, no stack arguments, no calls.
lf_checker_rt::export!(thiscall, rw_005ef280(this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x04;
        const COUNT: u32 = 0x018B7A50;
        let tls_block = lf_checker_rt::tls_slot(0);
        let link = ((this + HEAD) as *const u32).read_unaligned();
        let n = (lf_checker_rt::global::<u32>(COUNT)).read_unaligned();
        (lf_checker_rt::global::<u32>(COUNT)).write_unaligned(n.wrapping_sub(1));
        ((tls_block + HEAD) as *mut u32).write_unaligned(link);
        link
    }
});
