// original: 0x00d3ef50 CTaskComplexJump::vf19

/// Route a complex jump task to its next subtask (slot vf19).
///
/// `this` is the task, `owner` (arg0) the ped. The dword at
/// `LINK` (+0x20) of the owner points to a block whose dword at +0x38 is
/// copied to `CACHED` (+0x34). Then the mode word at `MODE` (+0x3c)
/// selects: bit 10 set calls the build callee (thiscall on `this`, tag
/// 0xfe, `owner`, `this`+0x40, the float and dwords at +0x50/+0x54/+0x58).
/// Otherwise the ped flag bytes at `F_A` (+0x26c) and `F_B` (+0x118) are
/// tested (bit 0 each; `F_A` is skipped when bit 9 of the mode is set):
/// both clear calls the switch callee (thiscall on `this`, tag 0xf1,
/// `owner`) after storing 0.75f (`TAG_VALUE`) at `BLEND` (+0x38),
/// otherwise the switch callee runs with tag 0xd2.
///
/// No return value. The pushed `this` overwritten by the float before the
/// build call is dead on both sides and passed as the float.
///
/// Original: 0x00d3ef50 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d3ef50(this: u32, owner: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x20;
        const CACHED: u32 = 0x34;
        const MODE: u32 = 0x3c;
        const MODE_BUILD: u16 = 0x400;
        const MODE_FAST: u16 = 0x200;
        const F_A: u32 = 0x26c;
        const F_B: u32 = 0x118;
        const BLEND: u32 = 0x38;
        const TAG_VALUE: u32 = 0x3f40_0000;
        const BUILD: u32 = 1;
        const SWITCH: u32 = 2;

        let link = ((owner + LINK) as *const u32).read_unaligned();
        ((this + CACHED) as *mut u32)
            .write_unaligned(((link + 0x38) as *const u32).read_unaligned());
        let mode = ((this + MODE) as *const u16).read_unaligned();
        if mode & MODE_BUILD != 0 {
            lf_checker_rt::callee_thiscall!(
                BUILD, u32, this, 0xfe, owner, this + 0x40,
                ((this + 0x50) as *const u32).read_unaligned(),
                ((this + 0x54) as *const u32).read_unaligned(),
                ((this + 0x58) as *const u32).read_unaligned()
            );
        } else if mode & MODE_FAST == 0
            && ((owner + F_A) as *const u8).read() & 1 != 0
        {
            lf_checker_rt::callee_thiscall!(SWITCH, u32, this, 0xd2, owner);
        } else if ((owner + F_B) as *const u8).read() & 1 != 0 {
            lf_checker_rt::callee_thiscall!(SWITCH, u32, this, 0xd2, owner);
        } else {
            ((this + BLEND) as *mut u32).write_unaligned(TAG_VALUE);
            lf_checker_rt::callee_thiscall!(SWITCH, u32, this, 0xf1, owner);
        }
        0
    }
});
