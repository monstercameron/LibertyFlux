// original: 0x00cf7fd0 CTaskComplexClimbLadderFully::vf1

/// Creates the child climb task for a full ladder task: allocates through the
/// game allocator, constructs the child with the word at `+0x14`, and returns
/// the constructor's result, or null when allocation fails.
///
/// Original: 0x00cf7fd0 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf7fd0(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167e2a0;
        const CHILD_PARAM: u32 = 0x14;
        const NEW_CALLEE: u32 = 1;
        const CTOR_CALLEE: u32 = 2;
        let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(NEW_CALLEE, u32, heap);
        if obj == 0 {
            return 0;
        }
        let param = ((this + CHILD_PARAM) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(CTOR_CALLEE, u32, obj, param)
    }
});
