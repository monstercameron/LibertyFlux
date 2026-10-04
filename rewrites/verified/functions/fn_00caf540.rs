// original: 0x00caf540 CTaskComplexMoveWander::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/6) forwarding fields of `this` (ECX): a0 dword at `+0x18`, a1 dword at `+0x28`, a2 bit 0, a3 dword at `+0xA0`, a4 bit 1 and a5 bit 5 of the dword at `+0xB0`. Afterwards copies the dwords at `+0xA4`/`+0xA8` into the clone. On NULL the stores run through a null pointer and fault, like the original.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf540(this: u32) -> u32 {
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
            let w = dw(this, 0xB0);
            clone = lf_checker_rt::callee_thiscall!(CTOR, u32, obj, dw(this, 0x18),
                dw(this, 0x28), w & 1, dw(this, 0xA0), (w >> 1) & 1, (w >> 5) & 1);
        }
        ((clone.wrapping_add(0xA4)) as *mut u32).write_unaligned(dw(this, 0xA4));
        ((clone.wrapping_add(0xA8)) as *mut u32).write_unaligned(dw(this, 0xA8));
        clone
    }
});
