// original: 0x00e5f4c0 link_network_timer_node_b

/// link network timer node b.
lf_checker_rt::export!(cdecl, rw_00e5f4c0() -> u32 {
    const LIST_HEAD: u32 = 0x017acd24;
    const NODE_NEXT: u32 = 0x0110f04c;
    const NODE_HEAD: u32 = 0x0110f048;
    unsafe {
        let previous = lf_checker_rt::global::<u32>(LIST_HEAD).read_volatile();
        lf_checker_rt::global::<u32>(NODE_NEXT).write_volatile(previous);
        lf_checker_rt::global::<u32>(LIST_HEAD).write_volatile(lf_checker_rt::relocated(NODE_HEAD));
        previous
    }
});
