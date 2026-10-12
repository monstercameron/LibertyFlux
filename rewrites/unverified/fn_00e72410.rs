// original: 0x00E72410 clear_stream_node_chain
/// When the chain end marker is nonzero, detach the head, reset the two links and end marker, then tail-call the global release helper.
lf_checker_rt::export!(cdecl, rw_fn_00E72410() -> u32 {
    unsafe {
        const CHAIN_ROOT: u32 = 0x0150E278;
        const CHAIN_LINK: u32 = 0x0150E27C;
        const CHAIN_FIRST: u32 = 0x0150E280;
        const CHAIN_LAST: u32 = 0x0150E284;
        const CHAIN_END: u32 = 0x0150E288;

        if lf_checker_rt::global::<u32>(CHAIN_END).read_unaligned() != 0 {
            let detached_head = lf_checker_rt::global::<u32>(CHAIN_LINK).read_unaligned();
            let root = lf_checker_rt::relocated(CHAIN_ROOT);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, root, detached_head);
            lf_checker_rt::global::<u32>(CHAIN_FIRST).write(root);
            lf_checker_rt::global::<u32>(CHAIN_LINK).write(0);
            lf_checker_rt::global::<u32>(CHAIN_LAST).write(root);
            lf_checker_rt::global::<u32>(CHAIN_END).write(0);
        }
        lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(CHAIN_ROOT))
    }
});
