// original: 0x0097EF40 ped_task_release_voice_task

/// Releases the secondary object stored in the task at `0x19c`.
///
/// A null slot returns without calls. Otherwise, the task's mode globals can
/// enable a one-argument cleanup helper for the object's `+0xa4` resource;
/// the object release helper always follows. Both calls are represented by
/// checker stand-ins, and the task slot itself is left unchanged.
lf_checker_rt::export!(thiscall, rw_0097EF40(this: u32) -> () {
    const OBJECT_SLOT: u32 = 0x19C;
    const RESOURCE_OFFSET: u32 = 0xA4;
    const CLEANUP_CALL: u32 = 1;
    const RELEASE_CALL: u32 = 2;
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let object = unsafe { read_word(this.wrapping_add(OBJECT_SLOT)) };
    if object == 0 { return; }
    let resource = unsafe { read_word(object.wrapping_add(RESOURCE_OFFSET)) };
    lf_checker_rt::callee_cdecl!(CLEANUP_CALL, u32, resource);
    lf_checker_rt::callee_thiscall!(RELEASE_CALL, u32, object, 0);
});
