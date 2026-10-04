// original: 0x00b382a0 pool_scan_and_report
/// Scan the object pool from the top down, collecting live entries that
/// pass the active check along with their class byte, then report each
/// collected entry to the two-argument worker. Mirrors the stack-cookie
/// check call so the call sequence matches exactly.
lf_checker_rt::export!(cdecl, rw_b382a0() -> u32 {
    unsafe {
        let pool = lf_checker_rt::global::<u32>(0x18b6f10).read();
        let base = (pool as *const u32).read();
        let flags = (pool as *const u32).add(1).read();
        let mut count = (pool as *const u32).add(2).read();
        let stride = (pool as *const u32).add(3).read();
        let mut buf = [(0u32, 0u32); 199];
        let mut n: u32 = 0;
        while count != 0 {
            count -= 1;
            if (flags.wrapping_add(count) as *const u8).read() & 0x80 != 0 {
                continue;
            }
            let e = base.wrapping_add(stride.wrapping_mul(count));
            if e == 0 {
                continue;
            }
            let c = ((e.wrapping_add(0x6c)) as *const u32).read();
            if c != 0 && (c.wrapping_add(0x0e) as *const u8).read() != 0 {
                continue;
            }
            if lf_checker_rt::callee_thiscall!(2, u32, e) & 0xff == 0 {
                continue;
            }
            let cls = lf_checker_rt::callee_thiscall!(3, u32, e) & 0xff;
            buf[n as usize] = (e, cls);
            n += 1;
        }
        let mut i: u32 = 0;
        while i < n {
            let (e, cls) = buf[i as usize];
            lf_checker_rt::callee_cdecl!(4, u32, e, cls);
            i += 1;
        }
        let _cookie = lf_checker_rt::global::<u32>(0x1057fb4).read();
        lf_checker_rt::callee_cdecl!(6, u32,);
        0
    }
});
