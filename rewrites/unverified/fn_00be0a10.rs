// original: 0x00BE0A10 CTaskSimpleSayAudio::vf1
/// Clone a `CTaskSimpleSayAudio` task (vtable slot 1).
///
/// Loads the allocator from the global at `ALLOCATOR` and allocates a fresh
/// object; when allocation fails (null) returns 0. Otherwise runs the copy
/// constructor as a thiscall with the fresh object in ECX and arg0 = pointer this+0x1c, arg1 = arg2 = float bits +0x40, arg3 = +0x44, arg4 = float bits +0x48,
/// and returns the constructor's result. All float fields move as bits
/// (the original uses movss, never arithmetic). Original is thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_00be0a10(this: u32) -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const P_1C: u32 = 0x1C;
    const FLT_40: u32 = 0x40;
    const F_44: u32 = 0x44;
    const FLT_48: u32 = 0x48;
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
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, this.wrapping_add(P_1C), rd32(this + FLT_40), rd32(this + FLT_40), rd32(this + F_44), rd32(this + FLT_48))
    }
});
