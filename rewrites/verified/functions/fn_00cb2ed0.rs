// original: 0x00CB2ED0 CTaskComplexMoveGoToShelterAndWait::vf19

/// Send the ped to shelter, or keep waiting when already sheltered.
///
/// `this` is the complex task, `ped` the ped. When the sheltered flag
/// (`this+0x50` bit 1) is set, the wait subtask (0x3ae) is created and
/// returned. Otherwise any pending shelter search (`this+0x30`) is
/// released (callee 2, one word, caller cleans up; the slot keeps its
/// stale value) and the go-to-shelter subtask (0x11a) is created.
/// Creation goes through the factory (callee 1) as (id, ped).
///
/// Original: 0x00CB2ED0 (thiscall, receiver in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00CB2ED0(this: u32, ped: u32) -> u32 {
    unsafe {
        const CREATE_SUB: u32 = 1;
        const RELEASE_SEARCH: u32 = 2;
        const STATE: u32 = 0x50;
        const SHELTERED_BIT: u8 = 2;
        const SEARCH_SLOT: u32 = 0x30;
        const WAIT: u32 = 0x3ae;
        const GO_TO_SHELTER: u32 = 0x11a;
        if (this.wrapping_add(STATE) as *const u8).read_unaligned() & SHELTERED_BIT != 0 {
            return lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, WAIT, ped);
        }
        let search = (this.wrapping_add(SEARCH_SLOT) as *const u32).read_unaligned();
        if search != 0 {
            lf_checker_rt::callee_cdecl!(RELEASE_SEARCH, u32, search);
        }
        lf_checker_rt::callee_thiscall!(CREATE_SUB, u32, this, GO_TO_SHELTER, ped)
    }
});
