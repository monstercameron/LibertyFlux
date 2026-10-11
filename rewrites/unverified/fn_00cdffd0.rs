// original: 0x00CDFFD0 CTaskSimpleNMFallDown::vf28

/// Dispatch the FallDown task and, for a successful dispatch and a signed
/// status above 3, invoke the target's virtual action at slot `+0xf4` with the
/// bit pattern for `1.0`. The selector at `this+0x2c` maps exactly 4 to mode 9
/// and every other value to mode 8. The dispatcher returns a boolean in AL,
/// which this method returns. Its computed ECX value feeds a second argument
/// to the virtual action; the checker contract skips only that one argument
/// because the runtime cannot transport a callee's computed ECX result. The
/// virtual action's return value is ignored.
lf_checker_rt::export!(thiscall, rw_00cdffd0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_STATUS: u32 = 0x28;
        const TASK_SELECTOR: u32 = 0x2c;
        const DISPATCH_MODE: u32 = 1;
        const MODE_DEFAULT: u32 = 8;
        const MODE_SPECIAL: u32 = 9;
        const ACTION_WEIGHT_BITS: u32 = 0x3f80_0000;
        const UNSCRIPTED_CONTEXT: u32 = 0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let selector = unsafe { read_u32(this.wrapping_add(TASK_SELECTOR)) };
        let mode = if selector == 4 { MODE_SPECIAL } else { MODE_DEFAULT };
        let dispatched = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            this,
            ped,
            DISPATCH_MODE,
            mode,
        ) as u8;

        let status = unsafe { read_u32(this.wrapping_add(TASK_STATUS)) } as i32;
        if dispatched != 0 && status > 3 {
            let _ignored_result = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                ped,
                ACTION_WEIGHT_BITS,
                UNSCRIPTED_CONTEXT,
            );
        }
        u32::from(dispatched)
    }
});
