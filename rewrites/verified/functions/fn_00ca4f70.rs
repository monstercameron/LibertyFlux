// original: 0x00ca4f70 CEventHandler::vf54
/// Event slot: reads the child pointer at event+0xC; when non-null, asks the
/// global factory for a handler and converts the child through it, storing
/// the result at this+0xC. Returns the event pointer, or the stored value.
lf_rs75_rt::export!(thiscall, rw_00ca4f70(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let child = *((ev + 0xC) as *const u32);
        if child == 0 {
            return ev;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = lf_rs75_rt::callee_thiscall!(2, u32, h, child);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
