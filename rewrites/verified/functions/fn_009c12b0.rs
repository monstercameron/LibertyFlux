// original: 0x009C12B0 ped_task_replace_owner_object
/// Replaces the object pointer at byte offset 0x25c of the receiver. When the
/// prior pointer is nonnull and differs from the replacement, it calls the
/// prior object's thiscall release helper and then the cdecl deallocator
/// before storing the new pointer. Null and equal-pointer cases only store.
/// The function uses 32-bit MSVC thiscall and has no semantic return value.
lf_checker_rt::export!(thiscall, rw_009c12b0(this: u32, new_object: u32) -> () {
    const OWNER_OBJECT_OFFSET: u32 = 0x25C;
    let owner_slot = this.wrapping_add(OWNER_OBJECT_OFFSET);
    let previous_object = unsafe { (owner_slot as *const u32).read() };

    if previous_object != 0 && previous_object != new_object {
        let _ = lf_checker_rt::callee_thiscall!(1, u32, previous_object);
        let _ = lf_checker_rt::callee_cdecl!(2, u32, previous_object);
    }

    unsafe { (owner_slot as *mut u32).write(new_object) };
});
