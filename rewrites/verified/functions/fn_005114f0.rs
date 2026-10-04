// original: 0x005114f0 ctor_Ranked_Episodic_Race_111
/// Constructor for the ranked episodic race 111 leaderboard info object.
///
/// Runs the shared base constructor on `this`, then installs this class's
/// two vtables, sets the initialised flag bit, stores the empty-slot marker
/// (-1) and clears the five trailing state words. Returns `this`.
export!(thiscall, rw_005114f0(this_ptr: u32) -> u32 {
    unsafe {
        // Base-class constructor (thiscall/0), intercepted by the checker.
        callee_thiscall!(1, u32, this_ptr);
        const VTABLE: u32 = 0x00fd0bf4;
        const VTABLE_INNER: u32 = 0x00fd6864;
        const EMPTY_SLOT: u32 = 0xffff_ffff;
        (this_ptr as *mut u32).write(relocated(VTABLE));
        (this_ptr.wrapping_add(0x4a0) as *mut u32).write(relocated(VTABLE_INNER));
        let flags = this_ptr.wrapping_add(0x5a4) as *mut u8;
        flags.write(flags.read() | 1);
        (this_ptr.wrapping_add(0x4a4) as *mut u32).write(EMPTY_SLOT);
        (this_ptr.wrapping_add(0x4a8) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4ac) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b0) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b4) as *mut u32).write(0);
        (this_ptr.wrapping_add(0x4b8) as *mut u32).write(0);
        this_ptr
    }
});
