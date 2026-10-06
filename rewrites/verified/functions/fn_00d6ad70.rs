// original: 0x00d6ad70 replay_reset_state_flags
/// Reset the state flags of the owned member, then notify the global owner
/// (original 0x00D6AD70, thiscall/0).
///
/// Writes 0 to byte `member+0x18` and 1 to byte `member+0x1a`, where `member`
/// is the dword at `this+4`; then calls the single-argument method (callee 1)
/// on the object held in the global at file VA 0x011F6954, passing 0.
/// Returns nothing meaningful.
lf_checker_rt::export!(thiscall, rw_00d6ad70(this_ptr: u32) -> u32 {
    unsafe {
        const MEMBER_OFF: u32 = 4;
        const FLAG_A_OFF: u32 = 0x18;
        const FLAG_B_OFF: u32 = 0x1a;
        const OWNER_GLOBAL: u32 = 0x011F6954;
        let member = ((this_ptr + MEMBER_OFF) as *const u32).read_unaligned();
        ((member + FLAG_A_OFF) as *mut u8).write(0);
        ((member + FLAG_B_OFF) as *mut u8).write(1);
        let owner =
            (lf_checker_rt::relocated(OWNER_GLOBAL) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, owner, 0);
        0
    }
});
