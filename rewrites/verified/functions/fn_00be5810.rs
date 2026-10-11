// original: 0x00BE5810 task_callback_forward

/// Fetch a callback object through the task's virtual method at offset 0x30,
/// invoke that object's virtual method at offset 0x1c with the stack argument,
/// then return the original argument unchanged. Both virtual calls are modeled
/// by planted function-table stubs in the proof contract.
lf_checker_rt::export!(thiscall, rw_00be5810(this: u32, argument: u32) -> u32 {
    unsafe {
        let first_object = ((this + 0x14) as *const u32).read_unaligned();
        let first_vtable = (first_object as *const u32).read_unaligned();
        let first_method = ((first_vtable + 0x30) as *const u32).read_unaligned();
        let get_callback: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(first_method as usize);
        let callback_object = get_callback(first_object);

        let callback_vtable = (callback_object as *const u32).read_unaligned();
        let callback_method = ((callback_vtable + 0x1c) as *const u32).read_unaligned();
        let run_callback: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callback_method as usize);
        let _ignored = run_callback(callback_object, argument);
        argument
    }
});
