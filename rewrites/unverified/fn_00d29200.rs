// original: 0x00d29200 targeting_bind_aux (proposed)

/// Bind the auxiliary handle and refresh the entry.
///
/// Runs the entry reset helper (intercepted) on `this`, stores `arg` at
/// `+0x238`, and when `arg` is nonzero retains the slot with the retain
/// helper (intercepted) and refreshes the entry with the refresh helper
/// (intercepted). The original leaves `eax` untouched, so no return channel
/// is compared.
///
/// Original: 0x00D29200 (thiscall, one stack argument).
lf_checker_rt::export!(thiscall, rw_00d29200(this: u32, arg: u32) -> u32 {
    unsafe {
        const SLOT_OFF: u32 = 0x238;
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this);
        let slot = this + SLOT_OFF;
        unsafe { (slot as *mut u32).write_unaligned(arg) };
        if arg != 0 {
            let _: u32 = lf_checker_rt::callee_stdcall!(2, u32, slot);
            let kept = unsafe { (slot as *const u32).read_unaligned() };
            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, this, kept);
        }
        0
    }
});
