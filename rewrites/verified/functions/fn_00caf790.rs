// original: 0x00caf790 CTaskSimpleMoveNone::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/1) forwarding fields of `this` (ECX): a0 immediate 1. Afterwards stamps the two vtable pointers (relocated image addresses) `0x00ED8DDC`/`0x00ED8E30` into the clone at `+0x00`/`+0x14` exactly as and returns the new object itself, not the constructor's answer. Incoming ECX is ignored. NULL returns NULL.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf790(this: u32) -> u32 {
    unsafe {
        const NEW: u32 = 1;
        const CTOR: u32 = 2;
        const ALLOCATOR_GLOBAL: u32 = 0x0167_E2A0;
        const VTBL0: u32 = 0x00ED_8DDC;
        const VTBL1: u32 = 0x00ED_8E30;
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
        let _: u32 = lf_checker_rt::callee_thiscall!(CTOR, u32, obj, 1);
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTBL0));
        ((obj.wrapping_add(0x14)) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTBL1));
        obj
    }
});
