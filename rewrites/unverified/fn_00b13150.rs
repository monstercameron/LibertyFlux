// original: 0x00B13150 CCutsceneObject::vf13

/// Return the cutscene object's area-clear status as a Boolean in AL.
///
/// If the optional child at `this+0x6c` exists, its byte at `+0x0e` must be
/// zero and its virtual slot 24 must return nonzero before the area state is
/// checked. The switch table returns false for states 2, 5, 6 and 7; every
/// other byte value returns true. The call uses the child as ECX, and the
/// function uses the Microsoft x86 thiscall convention with no stack args.
lf_checker_rt::export!(thiscall, rw_00B13150(this: u32) -> u32 {
    unsafe {
        const CHILD_SLOT: u32 = 0x6C;
        const CHILD_FLAGS: u32 = 0x0E;
        const AREA_STATE: u32 = 0x22A;

        let child = (this.wrapping_add(CHILD_SLOT) as *const u32).read_unaligned();
        if child != 0 {
            let flags = (child.wrapping_add(CHILD_FLAGS) as *const u8).read();
            if flags == 0 {
                let allowed = lf_checker_rt::callee_thiscall!(1, u32, child) as u8;
                if allowed == 0 {
                    return 0;
                }
            }
        }

        let state = (this.wrapping_add(AREA_STATE) as *const u8).read();
        if matches!(state, 2 | 5 | 6 | 7) { 0 } else { 1 }
    }
});
