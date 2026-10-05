// original: 0x00AF7000 veh_node_apply_logged_params (proposed)

/// Apply logged parameters for each node of a chain.
///
/// Walks the node chain whose head is at `list + 0x1C`, following `+0x18`
/// links. For each node, callee 1 classifies its head word; a nonzero
/// verdict skips the node. On zero, callee 2 derives an integer parameter
/// (kept at `out + 0x70`) and callee 3 derives a float parameter (kept at
/// `out + 0x74`). The pushed tag constants are relocated addresses of the
/// original's logging tags. Nothing is returned.
///
/// Original: 0x00AF7000 (cdecl, two stack arguments).
lf_checker_rt::export!(cdecl, rw_00AF7000(list: u32, out: u32) -> u32 {
    unsafe {
        const CLASSIFY: u32 = 1;
        const INT_PARAM: u32 = 2;
        const FLOAT_PARAM: u32 = 3;
        const TAG_CLASSIFY: u32 = 0xEA89C8;
        const TAG_INT: u32 = 0xEA89D0;
        const TAG_FLOAT: u32 = 0xEA89DC;
        const HEAD: u32 = 0x1C;
        const NEXT: u32 = 0x18;
        const OUT_INT: u32 = 0x70;
        const OUT_FLOAT: u32 = 0x74;
        let mut node = ((list + HEAD) as *const u32).read_unaligned();
        while node != 0 {
            let head = (node as *const u32).read_unaligned();
            let verdict = lf_checker_rt::callee_cdecl!(CLASSIFY, u32, head, lf_checker_rt::relocated(TAG_CLASSIFY));
            if verdict == 0 {
                let ip = lf_checker_rt::callee_thiscall!(INT_PARAM, u32, node, lf_checker_rt::relocated(TAG_INT), 0u32);
                ((out + OUT_INT) as *mut u32).write_unaligned(ip);
                let fp: f32 = lf_checker_rt::callee_stdcall!(FLOAT_PARAM, f32, lf_checker_rt::relocated(TAG_FLOAT), 0u32);
                ((out + OUT_FLOAT) as *mut u32).write_unaligned(fp.to_bits());
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
        0
    }
});
