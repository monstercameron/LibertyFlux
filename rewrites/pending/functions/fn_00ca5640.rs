// original: 0x00ca5640 CEventHandler::vf14
/// Byte-tagged event slot: combines the tag byte at event+0xC with the
/// event pointer's own high bytes (the original stages the byte in its dead
/// incoming stack slot; the rewrite computes the same dword directly) and
/// converts through the factory with the event id at +0x10. Stores the
/// result at this+0xC.
lf_rs75_rt::export!(thiscall, rw_00ca5640(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let id = *((ev + 0x10) as *const u32);
        let tag = *((ev + 0xC) as *const u8);
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let staged = (ev & 0xFFFFFF00) | (tag as u32);
        let ans: u32 = lf_rs75_rt::callee_thiscall!(2, u32, h, id, staged, 0);
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
