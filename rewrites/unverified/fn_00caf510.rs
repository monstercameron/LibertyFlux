// original: 0x00caf510 CTaskComplexMoveWaitForTraffic::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/2 with the new object in
/// ECX) forwarding these fields of `this` (ECX) as stack arguments: a0 pointer `this+0x40`; a1 pointer `this+0x50`.
/// The constructor's answer is the return value; a NULL allocator answer is
/// returned as NULL without calling the constructor. Original is
/// thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf510(this: u32) -> u32 {
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
        lf_checker_rt::callee_thiscall!(CTOR, u32, obj, this.wrapping_add(0x40), this.wrapping_add(0x50))
    }
});
