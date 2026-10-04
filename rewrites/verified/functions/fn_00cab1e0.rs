// original: 0x00CAB1E0 task_shift_spare (proposed)

/// Release the spare task, then move the current task into the spare slot.
///
/// When the spare-task slot at `this+0x0C` is non-null, calls virtual slot 0
/// of that object with argument 1 (a releasing call) with the object as
/// `this`. Afterwards the current-task pointer at `this+0x04` is copied into
/// the spare slot unconditionally. No stack arguments, no return value.
///
/// Original: 0x00CAB1E0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00cab1e0(this: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x04;
        const SPARE: u32 = 0x0C;
        const RELEASE_SLOT: u32 = 0x00;
        const RELEASE_ARG: u32 = 1;
        let spare = (this.wrapping_add(SPARE) as *const u32).read_unaligned();
        if spare != 0 {
            let vtable = (spare as *const u32).read_unaligned();
            let slot = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            release(spare, RELEASE_ARG);
        }
        let current = (this.wrapping_add(CURRENT) as *const u32).read_unaligned();
        (this.wrapping_add(SPARE) as *mut u32).write_unaligned(current);
        0
    }
});
