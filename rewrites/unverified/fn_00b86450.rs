// original: 0x00B86450 dist_probe_a
/// Probe the shared distance checker for `key`'s object.
///
/// Resolves the object (callee 1) and the gate (callee 2); when either
/// is missing the answer is 0. Otherwise a frame point is filled
/// (callee 3) and tested (callee 4): 1 on a hit, else the (zero) flag.
///
/// Original: 0x00B86450 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_00B86450(key: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x011D78F8;
        const GATE: u32 = 0x01632C60;
        let g = (lf_checker_rt::global::<u32>(GATE)).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(1, u32, g, key);
        let gate = lf_checker_rt::callee_cdecl!(2, u32,);
        let flag = 0u32;
        if gate & 0xFF == 0 || obj == 0 {
            return flag;
        }
        let mut pt = [0u32; 4];
        lf_checker_rt::callee_cdecl!(3, u32, pt.as_mut_ptr() as u32);
        let hit = lf_checker_rt::callee_thiscall!(4, u32, TABLE, obj, pt.as_ptr() as u32);
        if hit & 0xFF != 0 { 1 } else { flag }
    }
});
