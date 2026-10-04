// original: 0x005109b0 rlConcreteLeaderboardInfo_Race_81_ctor
/// Constructor for the Race_81 ranked-episodic leaderboard info object.
///
/// Runs the shared leaderboard-info base constructor on the object, then
/// installs this instantiation's primary and secondary virtual tables, marks
/// the secondary header slot empty, clears its trailing words and sets the
/// initialised flag bit. Returns the object pointer.
export!(thiscall, rw_005109b0(this_ptr: u32) -> u32 {
    unsafe {
        /// File VA of this instantiation's primary virtual table.
        const PRIMARY_VTABLE: u32 = 0x00FCF314;
        /// Byte offset of the secondary base's header within the object.
        const SECONDARY_OFF: usize = 0x4A0;
        /// File VA of this instantiation's secondary virtual table.
        const SECONDARY_VTABLE: u32 = 0x00FCFCC4;
        /// Empty-slot marker stored after the secondary virtual table.
        const EMPTY_SLOT: u32 = 0xFFFF_FFFF;
        /// Byte offset of the initialised flag.
        const FLAG_OFF: usize = 0x5A4;
        /// Bit set in the flag byte by construction.
        const FLAG_INITIALISED: u8 = 0x01;

        // Shared base constructor (scripted by the checker as callee 0).
        callee_thiscall!(0, u32, this_ptr);

        let obj = this_ptr as *mut u8;
        // Primary virtual table for this instantiation.
        (obj as *mut u32).write(relocated(PRIMARY_VTABLE));
        // Secondary base: its virtual table, an empty marker, then five
        // cleared words.
        let secondary = obj.add(SECONDARY_OFF) as *mut u32;
        secondary.write(relocated(SECONDARY_VTABLE));
        secondary.add(1).write(EMPTY_SLOT);
        for i in 2..7 {
            secondary.add(i).write(0);
        }
        // Initialised flag.
        *obj.add(FLAG_OFF) |= FLAG_INITIALISED;
        this_ptr
    }
});
