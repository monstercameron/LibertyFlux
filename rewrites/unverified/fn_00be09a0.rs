// original: 0x00BE09A0 CTaskSimpleMoveStandStill::vf1
/// Clone a `CTaskSimpleMoveStandStill` task (vtable slot 1).
///
/// Loads the allocator from the global at `ALLOCATOR` and allocates a fresh
/// object; when allocation fails (null) returns 0. Otherwise runs the copy
/// constructor as a thiscall with the fresh object in ECX and arg0 = +0x30, arg1 = zero-extended byte +0x38, arg2 = zero-extended byte +0x39, arg3 = float bits +0x34,
/// and returns the constructor's result. All float fields move as bits
/// (the original uses movss, never arithmetic). Original is thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_00be09a0(this: u32) -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const F_30: u32 = 0x30;
    const B_38: u32 = 0x38;
    const B_39: u32 = 0x39;
    const FLT_34: u32 = 0x34;
    #[inline(always)]
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn rd8(a: u32) -> u8 {
        unsafe { (a as *const u8).read() }
    }
    unsafe {

        let allocator = lf_checker_rt::global::<u32>(ALLOCATOR).read();
        let fresh = lf_checker_rt::callee_thiscall!(ALLOC, u32, allocator);
        if fresh == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, rd32(this + F_30), rd8(this + B_38) as u32, rd8(this + B_39) as u32, rd32(this + FLT_34))
    }
});
