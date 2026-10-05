// original: 0x00c48440 CCamScripted::vf7 (symbols)
/// Shut down: disable when the mode bit is set, release all three slots.
///
/// When flag bit 2 of `this + FLAGS` is set the member is disabled
/// first (callee 1). Then each occupied slot at `this + SLOT0/1/2` is
/// released (callees 2, 3, 4, each called with the occupant in ecx and
/// the slot address on the stack). Returns 1 in the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c48440(this: u32) -> u32 {
    const FLAGS: u32 = 0x264;
    const MODE_BIT: u8 = 4;
    const SLOTS: [u32; 3] = [0x240, 0x244, 0x248];
    const DISABLE: u32 = 1;
    const RELEASE0: u32 = 2;
    const RELEASE1: u32 = 3;
    const RELEASE2: u32 = 4;
    unsafe {
        if ((this + FLAGS) as *const u8).read() & MODE_BIT != 0 {
            lf_checker_rt::callee_thiscall!(DISABLE, u32, this);
        }
        let ids = [RELEASE0, RELEASE1, RELEASE2];
        let mut k = 0usize;
        while k < 3 {
            let slot = this + SLOTS[k];
            let occupant = (slot as *const u32).read_unaligned();
            if occupant != 0 {
                lf_checker_rt::callee_thiscall!(ids[k], u32, occupant, slot);
            }
            k += 1;
        }
    }
    1
});
