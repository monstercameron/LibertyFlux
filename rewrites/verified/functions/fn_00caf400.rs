// original: 0x00caf400 CTaskComplexMoveGoToPointRelativeToEntityAndStandStill::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/5 with the new object in
/// ECX) forwarding these fields of `this` (ECX) as stack arguments: a0 dword at `+0x40`; a1 dword at `+0x18`; a2 pointer `this+0x20`; a3 dword at `+0x44`; a4 dword at `+0x48`.
/// The constructor's answer is the return value; a NULL allocator answer is
/// returned as NULL without calling the constructor. Original is
/// thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf400(this: u32) -> u32 {
    unsafe {
        const NEW: u32 = 1;
        const CTOR: u32 = 2;
        const ALLOCATOR_GLOBAL: u32 = 0x0167_E2A0;
        #[inline(always)]
        unsafe fn dw(this: u32, off: u32) -> u32 {
            unsafe { ((this.wrapping_add(off)) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn sx16(this: u32, off: u32) -> u32 {
            unsafe { ((this.wrapping_add(off)) as *const i16).read_unaligned() as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn zx8(this: u32, off: u32) -> u32 {
            unsafe { ((this.wrapping_add(off)) as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn rb(this: u32, off: u32) -> u8 {
            unsafe { ((this.wrapping_add(off)) as *const u8).read() }
        }

        let mgr = (lf_checker_rt::global::<u32>(ALLOCATOR_GLOBAL) as *const u32).read();
        let obj: u32 = lf_checker_rt::callee_thiscall!(NEW, u32, mgr);
        if obj == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(CTOR, u32, obj, dw(this, 0x40), dw(this, 0x18), this.wrapping_add(0x20), dw(this, 0x44), dw(this, 0x48))
    }
});
