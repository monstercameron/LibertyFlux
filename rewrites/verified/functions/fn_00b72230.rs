// original: 0x00b72230 CTaskSimpleCarOpenLockedDoorFromOutside::vf17

/// Task update: force the locked door, then report done once.
///
/// Returns 1 at once when the door handle at `this+0x1c` is null or the
/// done flag at `this+0x14` is set. With no opener stored at `this+0x18`,
/// runs the open callee (thiscall on `this`, one stack word: the ped) and
/// reports not done while the slot stays null (in the proof the stubbed
/// callee never sets it, so this path always ends here). Otherwise queries
/// the ped through its vtable (slot 0x128, thiscall, no words): a nonzero
/// low byte reports not done, as does a clear 0x4000 bit in the opener's
/// word at +0x74. Else runs attach (thiscall on the opener, one stack word:
/// `this`), then force (thiscall on the opener, one stack word: -4.0f),
/// flags the opener with 0x4000, clears the slot and reports done. The two
/// early paths preserve the caller's upper return bytes (the contract pins
/// entry eax); every other path sets eax from a known value.
///
/// Original: 0x00b72230 (thiscall, one stack word: the ped).
lf_checker_rt::export!(thiscall, rw_00b72230(this: u32, ped: u32) -> u32 {
    unsafe {
        const DOOR_OFF: u32 = 0x1c;
        const DONE_OFF: u32 = 0x14;
        const OPENER_OFF: u32 = 0x18;
        const OPENER_FLAG_OFF: u32 = 0x74;
        const READY_FLAG: u32 = 0x4000;
        const FORCE_SPEED: u32 = 0xc0800000; // -4.0f
        const OPEN: u32 = 1;
        const PED_QUERY: u32 = 2;
        const ATTACH: u32 = 3;
        const FORCE: u32 = 4;
        if ((this + DOOR_OFF) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        if ((this + DONE_OFF) as *const u8).read() != 0 {
            return 1;
        }
        let mut opener = ((this + OPENER_OFF) as *const u32).read_unaligned();
        if opener == 0 {
            let ans: u32 = lf_checker_rt::callee_thiscall!(OPEN, u32, this, ped);
            opener = ((this + OPENER_OFF) as *const u32).read_unaligned();
            if opener == 0 {
                return ans & 0xffff_ff00;
            }
        }
        let vans: u32 = lf_checker_rt::callee_thiscall!(PED_QUERY, u32, ped);
        if (vans as u8) != 0 {
            return vans & 0xffff_ff00;
        }
        if ((opener + OPENER_FLAG_OFF) as *const u32).read_unaligned() & READY_FLAG == 0 {
            return vans & 0xffff_ff00;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(ATTACH, u32, opener, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(FORCE, u32, opener, FORCE_SPEED);
        let slot = (opener + 4) as *mut u32;
        slot.write_unaligned(slot.read_unaligned() | READY_FLAG);
        ((this + OPENER_OFF) as *mut u32).write_unaligned(0);
        (opener & 0xffff_ff00) | 1
    }
});
