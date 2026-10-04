// original: 0x00d27d10 ped_targetting_construct (proposed)

/// Construct a ped-targetting object: run the base constructor, then install
/// the vtable and the auxiliary table pointer and clear the handle area.
///
/// Calls the base constructor (intercepted) on `this`, stores `arg` at
/// `+0x24c`, the (relocated) vtable address at `+0x00` and the (relocated)
/// auxiliary table address at `+0x240`, and zeroes ten words from `+0x244`
/// through `+0x26c`. Returns `this`.
///
/// Original: 0x00D27D10 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d27d10(this: u32, arg: u32) -> u32 {
    unsafe {
        const ARG_OFF: u32 = 0x24c;
        const VTABLE_VA: u32 = 0x00ee_1c00;
        const AUX_TABLE_VA: u32 = 0x00d2_85a0;
        const AUX_OFF: u32 = 0x240;
        const CLEAR_OFFS: [u32; 10] =
            [0x244, 0x248, 0x250, 0x254, 0x258, 0x25c, 0x260, 0x264, 0x268, 0x26c];
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        unsafe { ((this + ARG_OFF) as *mut u32).write_unaligned(arg) };
        unsafe { (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_VA)) };
        unsafe { ((this + AUX_OFF) as *mut u32).write_unaligned(lf_checker_rt::relocated(AUX_TABLE_VA)) };
        for off in CLEAR_OFFS {
            unsafe { ((this + off) as *mut u32).write_unaligned(0) };
        }
        this
    }
});
