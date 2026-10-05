// original: 0x00be7270 CTaskSimpleOpenDoor::vf17

/// Advance an open-door task, resolving the door lazily, and report done.
///
/// `this` points to the task, `ped` to the ped. Latches the done flag at
/// `+0x24` when bit 0x40000 of the ped flags at `+0x26c` or bit 0x8000000 at
/// `+0x29c` is set. While the door handle at `+0x18` is null and the task is
/// not done, resolves the handle through callee 1 (thiscall with `this`, one
/// word: the ped) when the cached target at `+0x1c` is -1, then either starts
/// the open (callee 2, thiscall with `this`, two words: ped and the target)
/// when the side at `+0x20` is 1 or 4, or latches done. Returns 0 while not
/// done; otherwise releases a non-null door handle (callee 3, one word: the
/// task, whose entry `ecx` is path-dependent so the call is modelled as
/// stdcall; then callee 4, thiscall on the handle, one word holding the
/// float -1000.0) and clears the handle. Returns 1 once done.
///
/// Original: thiscall, one stack word, the callee pops 4 bytes, returns `al`.
lf_checker_rt::export!(thiscall, rw_00be7270(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_FLAGS_A: u32 = 0x26c;
        const PED_FLAGS_B: u32 = 0x29c;
        const FLAG_A_BIT: u32 = 0x40000;
        const FLAG_B_BIT: u32 = 0x8000000;
        const OFF_DOOR: u32 = 0x18;
        const OFF_TARGET: u32 = 0x1c;
        const OFF_SIDE: u32 = 0x20;
        const OFF_DONE: u32 = 0x24;
        const RELEASE_ARG: u32 = 0xc47a0000; // -1000.0f
        const RESOLVE: u32 = 1;
        const START_OPEN: u32 = 2;
        const TEARDOWN: u32 = 3;
        const RELEASE: u32 = 4;

        let fa = ((ped + PED_FLAGS_A) as *const u32).read_unaligned();
        let fb = ((ped + PED_FLAGS_B) as *const u32).read_unaligned();
        if fa & FLAG_A_BIT != 0 || fb & FLAG_B_BIT != 0 {
            ((this + OFF_DONE) as *mut u8).write(1);
        }
        if ((this + OFF_DOOR) as *const u32).read_unaligned() == 0
            && ((this + OFF_DONE) as *const u8).read() == 0
        {
            if ((this + OFF_TARGET) as *const u32).read_unaligned() == 0xFFFFFFFF {
                let h: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, ped);
                ((this + OFF_TARGET) as *mut u32).write_unaligned(h);
            }
            let side = ((this + OFF_SIDE) as *const u32).read_unaligned();
            if side == 1 || side == 4 {
                let target = ((this + OFF_TARGET) as *const u32).read_unaligned();
                lf_checker_rt::callee_thiscall!(START_OPEN, u32, this, ped, target);
            } else {
                ((this + OFF_DONE) as *mut u8).write(1);
            }
        }
        if ((this + OFF_DONE) as *const u8).read() == 0 {
            return 0;
        }
        let door = ((this + OFF_DOOR) as *const u32).read_unaligned();
        if door != 0 {
            lf_checker_rt::callee_stdcall!(TEARDOWN, u32, this);
            lf_checker_rt::callee_thiscall!(RELEASE, u32, door, RELEASE_ARG);
            ((this + OFF_DOOR) as *mut u32).write_unaligned(0);
        }
        1
    }
});
