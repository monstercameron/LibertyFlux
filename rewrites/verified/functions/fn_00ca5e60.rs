// original: 0x00ca5e60 CEventHandler::vf55
/// Type-gated event slot: type 0xC8 stores null, type 0x25C converts through
/// the factory into this+0xC, any other type leaves this+0xC alone.
/// Returns the type word, or the conversion result on the taken path.
lf_rs75_rt::export!(thiscall, rw_00ca5e60(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let t = *((ev + 0x10) as *const u32);
        if t == 0xC8 {
            *((this + 0xC) as *mut u32) = 0;
            return t;
        }
        if t != 0x25C {
            return t;
        }
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = lf_rs75_rt::callee_thiscall!(2, u32, h);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
