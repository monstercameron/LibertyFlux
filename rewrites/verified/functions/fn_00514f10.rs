// original: 0x00514f10 leaderboard_race_266_ctor
/// Constructor for the ranked episodic-race 266 leaderboard info object.
///
/// Runs the shared base constructor (a stubbed outgoing call), installs this
/// class's two relocated vtable pointers (the tracked `__vtbl` field at
/// offset 0 and the member vtable at offset 0x4a0), sets the initialized bit
/// in the flag byte at 0x5a4 while preserving its other bits, marks the
/// selection word at 0x4a4 as none (-1), zeroes the five trailing words at
/// 0x4a8..0x4bc, and returns the object pointer.
export!(thiscall, rw_00514f10(this: u32) -> u32 {
    unsafe {
        /// File VA of this class's primary vtable (relocated at load).
        const VTABLE: u32 = 0x00FDD4D4;
        /// File VA of the member vtable at offset 0x4a0 (relocated at load).
        const MEMBER_VTABLE: u32 = 0x00FD1D64;
        const MEMBER_OFF: usize = 0x4a0;
        const SELECTION_OFF: usize = 0x4a4;
        const FLAG_OFF: usize = 0x5a4;
        /// No entry selected.
        const NONE: u32 = 0xFFFF_FFFF;
        /// Bit set in the flag byte once the object is constructed.
        const INITIALIZED_BIT: u8 = 1;
        callee_thiscall!(2, u32, this);
        let obj = this as *mut u8;
        (obj as *mut u32).write(relocated(VTABLE));
        (obj.wrapping_add(MEMBER_OFF) as *mut u32).write(relocated(MEMBER_VTABLE));
        let flags = obj.wrapping_add(FLAG_OFF);
        flags.write(flags.read() | INITIALIZED_BIT);
        (obj.wrapping_add(SELECTION_OFF) as *mut u32).write(NONE);
        (obj.wrapping_add(0x4a8) as *mut u32).write(0);
        (obj.wrapping_add(0x4ac) as *mut u32).write(0);
        (obj.wrapping_add(0x4b0) as *mut u32).write(0);
        (obj.wrapping_add(0x4b4) as *mut u32).write(0);
        (obj.wrapping_add(0x4b8) as *mut u32).write(0);
        this
    }
});
