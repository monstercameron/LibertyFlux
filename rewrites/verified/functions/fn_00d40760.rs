// original: 0x00d40760 CTaskSimpleJumpLand::vf5

/// Decide whether a jump-land task accepts the requested transition.
///
/// `this` is the task, `owner` (arg0) the ped, `kind` (arg1, signed) the
/// requested transition, arg2 unused. Returns 0 when `kind` is below 1.
/// Otherwise, when the dword at `STATE` (+0x10) is nonzero, the settle
/// callee runs first (thiscall on `this` with `SETTLE_ARG`, -32.0f as bits);
/// then bit 14 of the ped flag word at `PED_FLAGS` (+0x26c) is cleared and
/// 1 is returned. Only al carries the result.
///
/// Original: 0x00d40760 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d40760(this: u32, owner: u32, kind: u32, _kind2: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x10;
        const SETTLE: u32 = 1;
        const SETTLE_ARG: u32 = 0xc100_0000;
        const PED_FLAGS: u32 = 0x26c;
        const KEEP_MASK: u32 = 0xffff_bfff;

        if (kind as i32) < 1 {
            return 0;
        }
        if ((this + STATE) as *const u32).read_unaligned() != 0 {
            lf_checker_rt::callee_thiscall!(SETTLE, u32, this, SETTLE_ARG);
        }
        let flags = (owner + PED_FLAGS) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & KEEP_MASK);
        1
    }
});
