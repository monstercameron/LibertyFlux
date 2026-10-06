// original: 0x006935E0 voice_hook_tailcall (proposed)

//
// Looks `arg` up with the intercepted table lookup, and tail-calls the
// function pointer at +8 of the entry found (a planted stub in the
// harness), returning its result; returns 0 when the lookup finds
// nothing. The jump is a computed tail jump through a register.
//
// Original: 0x006935E0 (cdecl, one stack argument).
lf_checker_rt::export!(cdecl, rw_006935E0(arg: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const LOOKUP: u32 = 1;
        let found = lf_checker_rt::callee_thiscall!(LOOKUP, u32, arg);
        if found == 0 {
            return 0;
        }
        let target = rd32(found.wrapping_add(8));
        let tail: extern "cdecl" fn() -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        tail()
    }
});
