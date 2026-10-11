// original: 0x00CDCB60 CTaskSimpleNMShot::vf1

/// Run the shot-task helper for this object. The `this` pointer is passed in
/// ECX and there are no stack arguments. A manager lookup is made first; a
/// null result returns zero without dispatching the task helper. Otherwise the
/// helper receives the lookup result plus the object's two 16-bit fields at
/// `+0x20` and `+0x22`, three words at `+0x54`, `+0x5c`, and `+0x58`, and
/// pointers to the three output records at `+0x60`, `+0x70`, and `+0x80`.
/// The return value is the helper's EAX result. The helper's own behavior is
/// represented by its checker stand-in, so this rewrite only owns the
/// argument selection and call order.
lf_checker_rt::export!(thiscall, rw_00cdcb60(this: u32) -> u32 {
    unsafe {
        const MANAGER_WORD: u32 = 0x0167_e2a0;
        const TASK_KIND: u32 = 0x20;
        const TASK_MODE: u32 = 0x22;
        const TASK_ARG_A: u32 = 0x54;
        const TASK_ARG_B: u32 = 0x5c;
        const TASK_ARG_C: u32 = 0x58;
        const TASK_OUT_A: u32 = 0x60;
        const TASK_OUT_B: u32 = 0x70;
        const TASK_OUT_C: u32 = 0x80;

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u32 {
            unsafe { (address as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = unsafe {
            lf_checker_rt::global::<u32>(MANAGER_WORD).read_unaligned()
        };
        let dispatch_context = lf_checker_rt::callee_thiscall!(1, u32, manager);
        if dispatch_context == 0 {
            return 0;
        }

        let task_kind = unsafe { read_u16(this.wrapping_add(TASK_KIND)) };
        let task_mode = unsafe { read_u16(this.wrapping_add(TASK_MODE)) };
        let task_arg_a = unsafe { read_u32(this.wrapping_add(TASK_ARG_A)) };
        let task_arg_b = unsafe { read_u32(this.wrapping_add(TASK_ARG_B)) };
        let task_arg_c = unsafe { read_u32(this.wrapping_add(TASK_ARG_C)) };
        let output_a = this.wrapping_add(TASK_OUT_A);
        let output_b = this.wrapping_add(TASK_OUT_B);
        let output_c = this.wrapping_add(TASK_OUT_C);

        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            dispatch_context,
            0,
            task_kind,
            task_mode,
            task_arg_a,
            task_arg_b,
            task_arg_c,
            output_a,
            output_b,
            output_c,
        )
    }
});
