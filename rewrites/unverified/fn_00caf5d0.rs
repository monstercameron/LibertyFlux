// original: 0x00caf5d0 CTaskComplexTurnToFaceEntityOrCoord::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (constructors A/B (intercepted callees 2/3, thiscall/3 and thiscall/3)) forwarding fields of `this` (ECX): the byte at `+0x18` selects the constructor: nonzero runs constructor A (a0 dword at `+0x14`, a1 dword at `+0x30`, a2 dword at `+0x34`), zero runs constructor B (a0 pointer `this+0x20`, a1 dword at `+0x30`, a2 dword at `+0x34`). Either returns the constructor's answer; NULL from the allocator returns NULL.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf5d0(this: u32) -> u32 {
    unsafe {
        const NEW: u32 = 1;
        const CTOR_A: u32 = 2;
        const CTOR_B: u32 = 3;
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
        if rb(this, 0x18) != 0 {
            let obj: u32 = lf_checker_rt::callee_thiscall!(NEW, u32, mgr);
            if obj == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(CTOR_A, u32, obj,
                dw(this, 0x14), dw(this, 0x30), dw(this, 0x34))
        } else {
            let obj: u32 = lf_checker_rt::callee_thiscall!(NEW, u32, mgr);
            if obj == 0 {
                return 0;
            }
            lf_checker_rt::callee_thiscall!(CTOR_B, u32, obj,
                this.wrapping_add(0x20), dw(this, 0x30), dw(this, 0x34))
        }
    }
});
