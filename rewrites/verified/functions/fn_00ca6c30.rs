// original: 0x00ca6c30 CEventHandler::vf67
/// Fixed-request event slot: converts request code 0x100 through the
/// factory and stores the result at this+0xC. Ignores its arguments.
lf_rs75_rt::export!(thiscall, rw_00ca6c30(this: u32, _a: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = lf_rs75_rt::callee_thiscall!(2, u32, h, 0x100);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
