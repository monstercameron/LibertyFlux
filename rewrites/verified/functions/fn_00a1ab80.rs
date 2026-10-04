// original: 0x00a1ab80 cam_dtor_members_tail (proposed)

/// Destroys the camera object: optional payload, members, chained tail.
///
/// When the flag byte at `+FLAG_OFF` is nonzero, the word at `+PAYLOAD_OFF`
/// is passed to the payload callee. Then the same member destructor runs
/// on the four members at `+M0_OFF`, `+M1_OFF`, `+M2_OFF`, `+M3_OFF` in
/// that order, and control passes by tail call to the member destructor
/// on `this` itself. Returns nothing.
///
/// Original: 0x00a1ab80 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00a1ab80(this: u32) -> u32 {
    unsafe {
        const C_PAYLOAD: u32 = 1;
        const C_MEMBER: u32 = 2;
        const FLAG_OFF: u32 = 0x58a;
        const PAYLOAD_OFF: u32 = 0x504;
        const M0_OFF: u32 = 0x2e0;
        const M1_OFF: u32 = 0x200;
        const M2_OFF: u32 = 0x140;
        const M3_OFF: u32 = 0x90;
        if ((this + FLAG_OFF) as *const u8).read() != 0 {
            let w = ((this + PAYLOAD_OFF) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(C_PAYLOAD, u32, w);
        }
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this + M0_OFF);
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this + M1_OFF);
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this + M2_OFF);
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this + M3_OFF);
        lf_checker_rt::callee_thiscall!(C_MEMBER, u32, this);
        0
    }
});
