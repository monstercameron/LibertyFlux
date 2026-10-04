// original: 0x00cf4340 CTaskSimpleClimb::vf1

/// Creates the worker for a simple climb task: allocates through the game
/// allocator and constructs it with five arguments (the word at `+0x64`, a
/// pointer to the embedded structure at `+0x20`, the float at `+0x30`, bit 0
/// of the flag byte at `+0x89`, the word at `+0x68`), returning the
/// constructor's result, or null when allocation fails.
///
/// Original: 0x00cf4340 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf4340(this: u32) -> u32 {
    unsafe {
        const ALLOCATOR_SLOT: u32 = 0x0167e2a0;
        const NEW_CALLEE: u32 = 1;
        const CTOR_CALLEE: u32 = 2;
        let heap = (lf_checker_rt::global::<u32>(ALLOCATOR_SLOT)).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(NEW_CALLEE, u32, heap);
        if obj == 0 {
            return 0;
        }
        let w64 = ((this + 0x64) as *const u32).read_unaligned();
        let w68 = ((this + 0x68) as *const u32).read_unaligned();
        let flag = (((this + 0x89) as *const u8).read() & 1) as u32;
        let f30 = ((this + 0x30) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(
            CTOR_CALLEE, u32, obj, w64, this.wrapping_add(0x20), f30, flag, w68)
    }
});
