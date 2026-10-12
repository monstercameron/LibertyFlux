// original: 0x00BE0B20 CTaskSimpleStandUp::vf1
/// Clone a `CTaskSimpleStandUp` task (vtable slot 1).
///
/// Loads the allocator from the global at `ALLOCATOR` and allocates a fresh
/// object; when allocation fails (null) returns 0. Otherwise runs the copy
/// constructor as a thiscall with the fresh object in ECX and arg0 = +0x18, arg1 = +0x1c,
/// and returns the constructor's result. All float fields move as bits
/// (the original uses movss, never arithmetic). Original is thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_00be0b20(this: u32) -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const F_18: u32 = 0x18;
    const F_1C: u32 = 0x1C;
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
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, rd32(this + F_18), rd32(this + F_1C))
    }
});
