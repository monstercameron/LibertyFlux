// original: 0x00caf380 CTaskComplexMoveGoToPointAndStandStill::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/6) forwarding fields of `this` (ECX): a0 dword at `+0x18`, a1 pointer `this+0x20`, a2 dword at `+0x30`, a3 dword at `+0x38`, a4 bit 0 and a5 bit 1 of the dword at `+0x3C`. Afterwards copies the dword at `+0x34` into the clone at `+0x34`. On NULL the store runs through a null pointer and faults, like the original.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf380(this: u32) -> u32 {
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
        let mut clone = 0u32;
        if obj != 0 {
            clone = lf_checker_rt::callee_thiscall!(CTOR, u32, obj, dw(this, 0x18),
                this.wrapping_add(0x20), dw(this, 0x30), dw(this, 0x38),
                dw(this, 0x3C) & 1, (dw(this, 0x3C) >> 1) & 1);
        }
        ((clone.wrapping_add(0x34)) as *mut u32).write_unaligned(dw(this, 0x34));
        clone
    }
});
