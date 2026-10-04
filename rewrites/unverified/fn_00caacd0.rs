// original: 0x00CAACD0 task_replace_aux (proposed)

/// Release the auxiliary task, then install a clone made by the new event.
///
/// When the auxiliary-task slot at `this+0x10` is non-null, calls virtual
/// slot 0 of that object with argument 1 (a releasing call). Then calls
/// slot `CLONE_SLOT` (0x10) of the second stack argument's virtual table
/// with that argument as `this`, and stores the result in the auxiliary
/// slot. The first stack argument is not read. No return value.
///
/// Original: 0x00CAACD0 (thiscall, two stack words, only the second read).
lf_checker_rt::export!(thiscall, rw_00caacd0(this: u32, _a0: u32, event: u32) -> u32 {
    unsafe {
        const AUX: u32 = 0x10;
        const RELEASE_SLOT: u32 = 0x00;
        const RELEASE_ARG: u32 = 1;
        const CLONE_SLOT: u32 = 0x10;
        let aux = (this.wrapping_add(AUX) as *const u32).read_unaligned();
        if aux != 0 {
            let vtable = (aux as *const u32).read_unaligned();
            let slot = (vtable.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            release(aux, RELEASE_ARG);
        }
        let vtable = (event as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(CLONE_SLOT) as *const u32).read_unaligned();
        let clone: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let task = clone(event);
        (this.wrapping_add(AUX) as *mut u32).write_unaligned(task);
        0
    }
});
