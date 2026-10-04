// original: 0x00b38210 route_and_follow_context_chain
/// Route `(a, f)` through the float or plain worker per the mode flag, then
/// follow the shared-context chain: bail out at any null or failed gate,
/// else finish with the two-argument worker. Returns the last answer seen.
lf_checker_rt::export!(cdecl, rw_b38210(a: u32, f: u32) -> u32 {
    unsafe {
        if lf_checker_rt::global::<u8>(0x104598e).read() == 0 {
            lf_checker_rt::callee_cdecl!(2, u32, a, f);
        } else {
            lf_checker_rt::callee_cdecl!(3, u32, a);
        }
        let r = lf_checker_rt::callee_cdecl!(4, u32,);
        if r == 0 {
            return 0;
        }
        let r2 = lf_checker_rt::callee_cdecl!(4, u32,);
        let x = (r2.wrapping_add(0x228) as *const u32).read();
        if x == 0 {
            return 0;
        }
        if x.wrapping_add(0x70) == 0 {
            return 0;
        }
        let r3 = lf_checker_rt::callee_cdecl!(4, u32,);
        let c = (r3.wrapping_add(0x228) as *const u32).read();
        let ecx = if c == 0 { 0 } else { c.wrapping_add(0x70) };
        let ans = lf_checker_rt::callee_thiscall!(5, u32, ecx);
        if (ans as i32) <= 0 {
            return ans;
        }
        let r4 = lf_checker_rt::callee_cdecl!(4, u32,);
        let q = (r4.wrapping_add(0x20) as *const u32).read().wrapping_add(0x30);
        lf_checker_rt::callee_cdecl!(6, u32, q, 0x42700000)
    }
});
