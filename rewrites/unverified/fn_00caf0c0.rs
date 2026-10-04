// original: 0x00caf0c0 CTaskComplexGoToPointAnyMeans::vf1
/// Clone the task (`vf1`): allocate a new object and construct a copy.
///
/// Allocates through the game allocator global at `0x0167E2A0` (intercepted
/// callee 1, thiscall/0) and, unless it returns NULL, runs the class
/// constructor (intercepted callee 2, thiscall/5) forwarding fields of `this` (ECX): a0 dword at `+0x30`, a1 pointer `this+0x20`, a2 dword at `+0x3C`, a3 dword at `+0x34`, a4 dword at `+0x40`. Afterwards copies the dword at `+0x38` into the clone at `+0x38` and merges bits 0-1 of the dword at `+0x50` into the clone's (xor/and/xor per bit). On NULL the stores run through a null pointer and fault, like the original.
/// Original is thiscall(`this`) and takes no stack arguments.
lf_checker_rt::export!(thiscall, rw_00caf0c0(this: u32) -> u32 {
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
            clone = lf_checker_rt::callee_thiscall!(CTOR, u32, obj, dw(this, 0x30),
                this.wrapping_add(0x20), dw(this, 0x3C), dw(this, 0x34), dw(this, 0x40));
        }
        ((clone.wrapping_add(0x38)) as *mut u32).write_unaligned(dw(this, 0x38));
        let slot = (clone.wrapping_add(0x50)) as *mut u32;
        let t = (zx8(this, 0x50) ^ slot.read_unaligned()) & 1;
        slot.write_unaligned(slot.read_unaligned() ^ t);
        let v = ((((dw(this, 0x50) >> 1) & 0xFF) << 1) ^ slot.read_unaligned()) & 2;
        slot.write_unaligned(slot.read_unaligned() ^ v);
        clone
    }
});
