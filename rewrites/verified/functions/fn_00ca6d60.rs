// original: 0x00ca6d60 CEventHandler::vf37
/// Type-clear event slot: when the event type word is 0xC8 or 0x201 it
/// clears this+0xC, otherwise it does nothing. Returns the type word.
/// No outgoing calls.
lf_rs75_rt::export!(thiscall, rw_00ca6d60(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let t = *((ev + 0x10) as *const u32);
        if t == 0xC8 || t == 0x201 {
            *((this + 0xC) as *mut u32) = 0;
        }
        t
    }
});
