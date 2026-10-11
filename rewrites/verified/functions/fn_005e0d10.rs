// original: 0x005E0D10 CTaskComplexCopHelicopter::CTaskComplexCopHelicopter

/// Initialize the helicopter cop task by running the base complex-task
/// constructor, installing this class's vtable, and clearing its two task
/// pointer fields at offsets +0x14 and +0x18. The base call is a thiscall with
/// no stack arguments; this constructor returns the original object pointer.
lf_checker_rt::export!(thiscall, rw_005e0d10(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00fe_0f9c;
        const FIRST_TASK: u32 = 0x14;
        const SECOND_TASK: u32 = 0x18;
        const BASE_CONSTRUCTOR: u32 = 1;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        write_u32(this, lf_checker_rt::relocated(VTABLE));
        write_u32(this.wrapping_add(FIRST_TASK), 0);
        write_u32(this.wrapping_add(SECOND_TASK), 0);
        this
    }
});
