// original: 0x00caf200 CTaskComplexMoveCrossRoadAtTrafficLights::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/5) forwarding fields of `this` (ECX): a0 pointer `this+0x20`, a1 pointer `this+0x30`, a2 `this+0x60` when bit 1 of the dword at `+0x74` is set else NULL, a3 dword at `+0x50`, a4 bit 2 of the dword at `+0x74`.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf200(this: u32) -> u32 {
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
        let flags = dw(this, 0x74);
        let side = if flags & 0x2 != 0 { this.wrapping_add(0x60) } else { 0 };
        let bit = (flags >> 2) & 1;
        lf_checker_rt::callee_thiscall!(CTOR, u32, obj, this.wrapping_add(0x20),
            this.wrapping_add(0x30), side, dw(this, 0x50), bit)
    }
});
