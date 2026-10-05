// original: 0x008e0270 pool_subsystem_init

/// Initialise the streaming pool subsystem: build the pool table, fill the
/// four geometry words by probing the allocator four times, publish the
/// 19-word subsystem descriptor (buffer geometry, handler table, sentinel),
/// then run the manager init on the descriptor's answer and publish that.
///
/// The probe loop writes the four answers to the geometry global and stops
/// when the cursor reaches the table global. The descriptor call cleans its
/// own stack (stdcall); the manager object is whatever it returned.
///
/// Original: cdecl, no arguments; returns the manager init's answer.
lf_checker_rt::export!(cdecl, rw_008e0270() -> u32 {
    unsafe {
        const TABLE_GEOM: u32 = 0x1770;
        lf_checker_rt::callee_cdecl!(
            1,
            u32,
            TABLE_GEOM,
            lf_checker_rt::relocated(0xE81E48)
        );
        let mut cursor = lf_checker_rt::relocated(0x11764B0) as *mut u32;
        let end = lf_checker_rt::relocated(0x11764C0) as *mut u32;
        let probe_arg = lf_checker_rt::relocated(0xE81E54);
        while cursor < end {
            *cursor = lf_checker_rt::callee_cdecl!(2, u32, probe_arg);
            cursor = cursor.add(1);
        }
        let ans = lf_checker_rt::callee_stdcall!(
            3,
            u32,
            lf_checker_rt::relocated(0xE81E5C),
            lf_checker_rt::relocated(0xE81E58),
            0x1770u32,
            lf_checker_rt::relocated(0x5CEB70),
            0u32,
            lf_checker_rt::relocated(0x8E0500),
            lf_checker_rt::relocated(0x8DFEA0),
            lf_checker_rt::relocated(0x8E0620),
            lf_checker_rt::relocated(0x8E0600),
            lf_checker_rt::relocated(0x8E0920),
            lf_checker_rt::relocated(0x5B6110),
            lf_checker_rt::relocated(0x5B6140),
            lf_checker_rt::relocated(0x8E0210),
            lf_checker_rt::relocated(0x8E0880),
            lf_checker_rt::relocated(0x8DFEE0),
            lf_checker_rt::relocated(0x8DFF60),
            0u32,
            1u32,
            8u32
        );
        let mgr_ans = lf_checker_rt::callee_thiscall!(4, u32, ans);
        *lf_checker_rt::global::<u32>(0x1032F58) = mgr_ans;
        mgr_ans
    }
});
