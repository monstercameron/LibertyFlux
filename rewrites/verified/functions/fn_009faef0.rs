// original: 0x009faef0 submit_playstat_9
/// Builds a temporary base play-statistic tagged 9 on the stack, runs it
/// through the shared submit step, destroys it and checks the stack cookie.
/// No result; the observable behaviour is the four calls in order.
export!(cdecl, rw_rs227_009faef0() -> u32 {
    unsafe {
        let mut obj = [0u32; 13];
        let p = obj.as_mut_ptr() as u32;
        callee_thiscall!(0, u32, p, 9);
        callee_cdecl!(1, u32, p, 0x34);
        callee_thiscall!(2, u32, p);
        callee_thiscall!(3, u32, *global::<u32>(0x0105_7FB4 /* stack-cookie slot */));
        0
    }
});
