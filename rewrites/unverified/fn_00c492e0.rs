// original: 0x00c492e0 CCamCinematic::vf7 (symbols)
/// Shut down: release the slot, then snapshot or clear the shared flag.
///
/// Releases the occupant of the slot at `this + SLOT` if any
/// (callee 1) and clears the slot. When the mode byte at `this + MODE`
/// is not 0, sets the shared active flag and snapshots `this` into the
/// shared camera block (callee 2, called with the block address in ecx
/// and `this` on the stack); otherwise clears the shared flag.
/// Returns 1 in the low byte.
///
/// The block address is an absolute file VA the original loads as an
/// immediate without a relocation entry; the rewrite reproduces the
/// same value (it is only passed to the scripted callee, never
/// dereferenced), so the call comparison sees identical arguments.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c492e0(this: u32) -> u32 {
    const SLOT: u32 = 0x1f0;
    const MODE: u32 = 0x206;
    const ACTIVE_FLAG: u32 = 0x016d8b40;
    // Unrelocated file VA, reproduced verbatim (see above).
    const SHARED_BLOCK: u32 = 0x016d8b50;
    const RELEASE: u32 = 1;
    const SNAPSHOT: u32 = 2;
    unsafe {
        let slot = this + SLOT;
        let occupant = (slot as *const u32).read_unaligned();
        if occupant != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, occupant, slot);
        }
        (slot as *mut u32).write_unaligned(0);
        if ((this + MODE) as *const u8).read() != 0 {
            lf_checker_rt::global::<u8>(ACTIVE_FLAG).write(1);
            lf_checker_rt::callee_thiscall!(SNAPSHOT, u32, SHARED_BLOCK, this);
        } else {
            lf_checker_rt::global::<u8>(ACTIVE_FLAG).write(0);
        }
    }
    1
});
