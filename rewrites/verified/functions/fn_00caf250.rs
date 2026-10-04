// original: 0x00caf250 CTaskComplexMoveCrowdAroundLocation::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/3) forwarding fields of `this` (ECX): a0 pointer `this+0x20`, a1 dword at `+0x50`, a2 the global float bits at `0x01050E84`. Afterwards copies the byte at `+0x58` into the clone at `+0x58`. On NULL the store runs through a null pointer and faults, like the original.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf250(this: u32) -> u32 {
    unsafe {
        const NEW: u32 = 1;
        const CTOR: u32 = 2;
        const ALLOCATOR_GLOBAL: u32 = 0x0167_E2A0;
        const ANGLE_CONST: u32 = 0x0105_0E84;
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
            let angle = (lf_checker_rt::global::<u32>(ANGLE_CONST) as *const u32).read();
            clone = lf_checker_rt::callee_thiscall!(CTOR, u32, obj,
                this.wrapping_add(0x20), dw(this, 0x50), angle);
        }
        ((clone.wrapping_add(0x58)) as *mut u8).write(rb(this, 0x58));
        clone
    }
});
