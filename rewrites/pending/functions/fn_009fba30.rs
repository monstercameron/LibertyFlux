// original: 0x009fba30 submit_playstat_16
/// Extended temporary-statistic submit tagged 16.
///
/// Gathers a 64-bit stamp, folds the shared step's result into its high
/// half, forwards both halves, runs two parameterless steps, then builds a
/// temporary base statistic tagged 16, submits and destroys it, and checks
/// the stack cookie. The second argument of the first call is an
/// uninitialised register word in the original; the rewrite passes 0 and the
/// contract does not compare it.
export!(cdecl, rw_rs227_009fba30() -> u32 {
    unsafe {
        let mut buf = [0u32; 8];
        let b = buf.as_mut_ptr() as u32;
        callee_cdecl!(0, u32, b, 0);
        let mix = callee_cdecl!(1, u32, b, 6, 0);
        let stamp = callee_cdecl!(2, u64,);
        let lo = stamp as u32;
        let hi = (stamp >> 32) as u32;
        callee_cdecl!(3, u32, lo, hi.wrapping_add(mix));
        callee_cdecl!(4, u32,);
        callee_cdecl!(5, u32,);
        let mut obj = [0u32; 13];
        let p = obj.as_mut_ptr() as u32;
        callee_thiscall!(6, u32, p, 0x10);
        callee_cdecl!(7, u32, p, 0x34);
        callee_thiscall!(8, u32, p);
        callee_thiscall!(9, u32, *global::<u32>(0x0105_7FB4 /* stack-cookie slot */));
        0
    }
});
