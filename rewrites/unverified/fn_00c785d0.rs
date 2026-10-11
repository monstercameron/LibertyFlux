// original: 0x00C785D0 phbound_state_init (proposed)

/// Initialize the 48-byte state block at `this`. The single stack argument
/// is an f32 bit pattern and is forwarded unchanged to a cdecl helper. The
/// source also copies an unwritten aligned stack slot to `+0x1c` and
/// `+0x2c`; this proof defines that slot as zero. The helper's EAX result is
/// stored at `+8` and returned. The block also receives zero at `+0` and
/// `+4`, positive maximum finite f32 values at `+0x10..+0x18`, and negative
/// maximum finite f32 values at `+0x20..+0x28`.
lf_checker_rt::export!(thiscall, rw_phbound_state_init(this: u32, value_bits: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        const HELPER_VALUE: u32 = 0x08;
        const POSITIVE_LIMIT: u32 = 0x7f7f_ffff;
        const NEGATIVE_LIMIT: u32 = 0xff7f_ffff;
        const POSITIVE_X: u32 = 0x10;
        const POSITIVE_Y: u32 = 0x14;
        const POSITIVE_Z: u32 = 0x18;
        const STACK_SPILL_COPY_A: u32 = 0x1c;
        const NEGATIVE_X: u32 = 0x20;
        const NEGATIVE_Y: u32 = 0x24;
        const NEGATIVE_Z: u32 = 0x28;
        const STACK_SPILL_COPY_B: u32 = 0x2c;
        const DEFINED_STACK_SPILL: u32 = 0;

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let helper_result = lf_checker_rt::callee_cdecl!(HELPER, u32, value_bits);
        unsafe {
            write_u32(this, 0);
            write_u32(this + 4, 0);
            write_u32(this + HELPER_VALUE, helper_result);
            write_u32(this + POSITIVE_X, POSITIVE_LIMIT);
            write_u32(this + POSITIVE_Y, POSITIVE_LIMIT);
            write_u32(this + POSITIVE_Z, POSITIVE_LIMIT);
            write_u32(this + STACK_SPILL_COPY_A, DEFINED_STACK_SPILL);
            write_u32(this + NEGATIVE_X, NEGATIVE_LIMIT);
            write_u32(this + NEGATIVE_Y, NEGATIVE_LIMIT);
            write_u32(this + NEGATIVE_Z, NEGATIVE_LIMIT);
            write_u32(this + STACK_SPILL_COPY_B, DEFINED_STACK_SPILL);
        }
        helper_result
    }
});
