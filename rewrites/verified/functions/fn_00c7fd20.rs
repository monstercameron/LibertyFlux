// original: 0x00c7fd20 CTaskComplexStationaryScenario::vf6

/// Scenario blend weight gated on the subtask reporting kind `0x123`.
///
/// Bit 1 of `this+0xc` set yields 25.0 at once. Otherwise the subtask at
/// `this+8` must be present and its kind (virtual slot `+0xc`) must read
/// `0x123`, and the object must be active (flag byte at `this+0x20` clear or
/// word at `this+0x1c` non-zero); then the weight is 25.0, else -1.0. The one
/// stack word is popped but never read. Returned in ST0.
///
/// Original: thiscall, one stack word (the callee pops 4 bytes), float result in ST0.
lf_checker_rt::export!(thiscall, rw_00c7fd20(this: u32, _u: u32) -> f32 {
    unsafe {
        const FLAG_OFF: u32 = 0x0c;
        const SUB_OFF: u32 = 8;
        const ACTIVE_OFF: u32 = 0x20;
        const STATE_OFF: u32 = 0x1c;
        const VT_KIND: u32 = 0x0c;
        const WANT_KIND: u32 = 0x123;
        const ACTIVE_W: f32 = 25.0;
        const IDLE_W: f32 = -1.0;
        let flags = ((this + FLAG_OFF) as *const u32).read_unaligned();
        if (flags >> 1) & 1 != 0 {
            return ACTIVE_W;
        }
        let sub = ((this + SUB_OFF) as *const u32).read_unaligned();
        if sub == 0 {
            return IDLE_W;
        }
        let vt = (sub as *const u32).read_unaligned();
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(((vt + VT_KIND) as *const u32).read_unaligned() as usize);
        if kind_of(sub) != WANT_KIND {
            return IDLE_W;
        }
        let flag = ((this + ACTIVE_OFF) as *const u8).read();
        let state = ((this + STATE_OFF) as *const u32).read_unaligned();
        if flag == 0 || state != 0 { ACTIVE_W } else { IDLE_W }
    }
});
