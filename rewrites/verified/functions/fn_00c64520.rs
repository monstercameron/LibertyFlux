// original: 0x00c64520 CCutsceneObject::create_draw_commands

/// Emit draw commands for the cutscene object according to its mode.
///
/// `this` is the cutscene object and `a0..a3` are four stack words (`a3`
/// is never read). The word at `MODE (+0x314)` selects the path; every test
/// is an equality test against a small constant, so signedness plays no
/// part, and every path returns 0:
/// - mode 0: helper 2 is called with (`this`, `a0`, `a1`), then helper 3
///   with (`this`, `a0`, `a1`).
/// - mode 1: helper 4 is called with (`this`, `a0`, `a1`, `a2`, -1); when
///   its answer is nonzero (equality with zero), bit 0 of the answer's
///   byte at `+0x59` is set.
/// - mode 2: the flag helper 1 (`is_field_2a0_nonzero` on `this`) is asked;
///   when it answers zero, helper 4 is called with (`this`, `a0`, `a1`,
///   `a2`, -1) and its answer is dropped.
/// - any other mode: nothing is called.
///
/// Original: thiscall, four stack words, callee pops them (the callee pops 0x10 bytes),
/// word result in EAX (always 0).
lf_checker_rt::export!(thiscall, rw_00c64520(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    const MODE: u32 = 0x314;
    const FLAG_BIT_OFF: u32 = 0x59;
    const HELPER_FLAG: u32 = 1;
    const HELPER_A: u32 = 2;
    const HELPER_B: u32 = 3;
    const HELPER_EMIT: u32 = 4;
    unsafe {
        let _ = a3;
        let mode = ((this + MODE) as *const u32).read_unaligned();
        if mode == 0 {
            lf_checker_rt::callee_thiscall!(HELPER_A, u32, this, a0, a1);
            lf_checker_rt::callee_thiscall!(HELPER_B, u32, this, a0, a1);
        } else if mode == 1 {
            let ans = lf_checker_rt::callee_thiscall!(
                HELPER_EMIT, u32, this, a0, a1, a2, 0xffff_ffff);
            if ans != 0 {
                let p = (ans + FLAG_BIT_OFF) as *mut u8;
                p.write_unaligned(p.read_unaligned() | 1);
            }
        } else if mode == 2 {
            let flag = lf_checker_rt::callee_thiscall!(HELPER_FLAG, u32, this);
            if flag & 0xff == 0 {
                lf_checker_rt::callee_thiscall!(
                    HELPER_EMIT, u32, this, a0, a1, a2, 0xffff_ffff);
            }
        }
        0
    }
});
