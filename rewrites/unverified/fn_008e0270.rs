// original: 0x008e0270 pool_subsystem_init

/// Initialise the streaming pool subsystem: build one pool table from the
/// two global dimensions, register the pool helper, publish the 19-word
/// subsystem descriptor (buffer geometry, handler table, sentinel), then run
/// the manager's own init and return its answer.
///
/// The key and value widths come from the two words at the fixed geometry
/// global; both are forwarded to the table builder as (value, key).
///
/// Original: cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008e0270() -> u32 {
    unsafe {
        let geom = lf_checker_rt::relocated(0x11764B0) as *const u32;
        let key_w = *geom;
        let val_w = *geom.add(1);
        lf_checker_rt::callee_cdecl!(1, u32, val_w, key_w);
        lf_checker_rt::callee_cdecl!(2, u32, *lf_checker_rt::global::<u32>(lf_checker_rt::relocated(0x1032F58)));
        let desc = lf_checker_rt::relocated(0xE81E5C) as u32;
        let extra = lf_checker_rt::relocated(0xE81E58) as u32;
        let cap = 0x1770u32;
        let h_probe = lf_checker_rt::relocated(0x5CEB70) as u32;
        let h_read = lf_checker_rt::relocated(0x8E0500) as u32;
        let h_free = lf_checker_rt::relocated(0x8DFEA0) as u32;
        let h_guard = lf_checker_rt::relocated(0x8E0620) as u32;
        let h_miss = lf_checker_rt::relocated(0x8E0600) as u32;
        let h_scan = lf_checker_rt::relocated(0x8E0920) as u32;
        let h_a = lf_checker_rt::relocated(0x5B6110) as u32;
        let h_b = lf_checker_rt::relocated(0x5B6140) as u32;
        let h_data = lf_checker_rt::relocated(0x8E0210) as u32;
        let h_row = lf_checker_rt::relocated(0x8E0880) as u32;
        let h_tls = lf_checker_rt::relocated(0x8DFEE0) as u32;
        let h_tail = lf_checker_rt::relocated(0x8DFF60) as u32;
        lf_checker_rt::callee_cdecl!(3, u32, desc, extra, cap, h_probe, 0u32, h_read, h_free,
            h_guard, h_miss, h_scan, h_a, h_b, h_data, h_row, h_tls, h_tail,
            0u32, 1u32, 8u32);
        let mgr = lf_checker_rt::relocated(0x1173750) as *mut u32;
        lf_checker_rt::callee_thiscall!(4, u32, mgr as u32)
    }
});
