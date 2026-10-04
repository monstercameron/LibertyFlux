// original: 0x00cf77f0 CTaskComplexClimbLadderFully::vf0

/// Deleting destructor for a full climb-ladder task: runs the destructor,
/// then frees the object through the game allocator when bit 0 of the flags
/// word is set. Returns the object pointer.
///
/// Original: 0x00cf77f0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf77f0(this: u32, flags: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167e2a0;
        const DTOR_CALLEE: u32 = 1;
        const DELETE_CALLEE: u32 = 2;
        lf_checker_rt::callee_thiscall!(DTOR_CALLEE, u32, this);
        if flags & 1 != 0 {
            let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
            lf_checker_rt::callee_thiscall!(DELETE_CALLEE, u32, heap, this);
        }
        this
    }
});
