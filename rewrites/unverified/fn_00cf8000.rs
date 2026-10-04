// original: 0x00cf8000 CTaskSimpleClimbLadder::vf1

/// Creates the worker for a simple climb-ladder task: allocates through the
/// game allocator and constructs it with seven arguments (the word at `+0x14`,
/// pointers to the embedded structures at `+0x30`, `+0x40` and `+0x70`, the
/// float at `+0x50`, the flag byte at `+0xcb`, the word at `+0x18`), returning
/// the constructor's result, or null when allocation fails.
///
/// Original: 0x00cf8000 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf8000(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167e2a0;
        const NEW_CALLEE: u32 = 1;
        const CTOR_CALLEE: u32 = 2;
        let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(NEW_CALLEE, u32, heap);
        if obj == 0 {
            return 0;
        }
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        let flag = ((this + 0xcb) as *const u8).read() as u32;
        let f50 = ((this + 0x50) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            CTOR_CALLEE, u32, obj, w14,
            this.wrapping_add(0x30), this.wrapping_add(0x40),
            this.wrapping_add(0x70), f50, flag, w18)
    }
});
