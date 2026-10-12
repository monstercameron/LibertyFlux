// original: 0x00BE0A90 CTaskSimpleSitDown::vf1
/// Clone a `CTaskSimpleSitDown` task (vtable slot 1).
///
/// Loads the allocator from the global at `ALLOCATOR` and allocates a fresh
/// object; when allocation fails (null) returns 0. Otherwise runs the copy
/// constructor as a thiscall with the fresh object in ECX and arg0..arg2 = +0x68/+0x6c/+0x64, arg3 = float bits +0x70, arg4 = this+0x40, arg5 = this+0x50, arg6 = this+0x20 when byte +0x75 is nonzero else 0,
/// and returns the constructor's result. All float fields move as bits
/// (the original uses movss, never arithmetic). Original is thiscall with
/// no stack words.
lf_checker_rt::export!(thiscall, rw_00be0a90(this: u32) -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const ALLOCATOR: u32 = 0x0167E2A0;
    const F_68: u32 = 0x68;
    const F_6C: u32 = 0x6C;
    const F_64: u32 = 0x64;
    const FLT_70: u32 = 0x70;
    const P_40: u32 = 0x40;
    const P_50: u32 = 0x50;
    const FLAG_OFF: u32 = 0x75;
    const ALT_PTR: u32 = 0x20;
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
        lf_checker_rt::callee_thiscall!(CTOR, u32, fresh, rd32(this + F_68), rd32(this + F_6C), rd32(this + F_64), rd32(this + FLT_70), this.wrapping_add(P_40), this.wrapping_add(P_50), (if rd8(this + FLAG_OFF) == 0 { 0 } else { this.wrapping_add(ALT_PTR) }))
    }
});
