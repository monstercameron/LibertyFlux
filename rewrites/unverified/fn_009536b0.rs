// original: 0x009536B0 resolve_handle_or_table (proposed)

/// Resolve a handle through two callees, else read a flagged table entry.
///
/// Probes `h` through callee 1 (one stack word); only the LOW byte of the
/// answer is tested. When non-zero, callee 2 resolves `h`: a null answer
/// returns 0, otherwise the dword it points to is returned. When the low
/// byte is zero, the low 16 bits of `h` are used as an UNSIGNED table index:
/// above `BOUND` the return is 0, otherwise the flag byte at `FLAGS + index`
/// decides whether callee 3 first initialises the 8-byte entry at
/// `TABLE + index * 8` (thiscall, the entry address in ECX), and the entry's
/// first dword is returned.
///
/// Original: 0x009536B0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009536B0(h: u32) -> u32 {
    unsafe {
        const BOUND: u32 = 0x5DB;
        const FLAGS: u32 = 0x11F6958;
        const TABLE: u32 = 0x11F7110;
        const CLASSIFY: u32 = 1;
        const LOOKUP: u32 = 2;
        const ENSURE: u32 = 3;
        let r1 = lf_checker_rt::callee_cdecl!(CLASSIFY, u32, h);
        if (r1 & 0xFF) != 0 {
            let p = lf_checker_rt::callee_cdecl!(LOOKUP, u32, h);
            if p == 0 {
                0
            } else {
                (p as *const u32).read_unaligned()
            }
        } else {
            let si = h & 0xFFFF;
            if si > BOUND {
                0
            } else {
                let flag =
                    (lf_checker_rt::relocated(FLAGS).wrapping_add(si) as *const u8).read();
                let entry = lf_checker_rt::relocated(TABLE).wrapping_add(si.wrapping_mul(8));
                if flag == 0 {
                    lf_checker_rt::callee_thiscall!(ENSURE, u32, entry);
                }
                (entry as *const u32).read()
            }
        }
    }
});
