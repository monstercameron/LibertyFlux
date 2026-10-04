// original: 0x00CAAF80 task_release_spare (proposed)

/// Release the spare task object and clear its slot.
///
/// When the spare-task slot at `this+0x0C` is non-null, calls virtual slot 0
/// of that object with argument 1 (a releasing call) with the object as
/// `this`, then clears the slot. A null spare means nothing happens. No
/// stack arguments, no return value.
///
/// Original: 0x00CAAF80 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00caaf80(this: u32) -> u32 {
    unsafe {
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
            (this.wrapping_add(SPARE) as *mut u32).write_unaligned(0);
        }
        0
    }
});
