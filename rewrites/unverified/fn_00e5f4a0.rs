// original: 0x00e5f4a0 link_network_timer_node_a

/// link network timer node a. The entry is cdecl with no stack arguments and returns the previous list head in EAX.
lf_checker_rt::export!(cdecl, rw_00e5f4a0() -> u32 {
    const LIST_HEAD: u32 = 0x017acd24;
    const NODE_NEXT: u32 = 0x0110f054;
    const NODE_HEAD: u32 = 0x0110f050;
    unsafe {
        let previous = lf_checker_rt::global::<u32>(LIST_HEAD).read_volatile();
        lf_checker_rt::global::<u32>(NODE_NEXT).write_volatile(previous);
        lf_checker_rt::global::<u32>(LIST_HEAD).write_volatile(lf_checker_rt::relocated(NODE_HEAD));
        previous
    }
});
