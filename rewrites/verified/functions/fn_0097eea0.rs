// original: 0x0097EEA0 ped_task_release_audio_task

/// Releases the secondary object stored in the task at `0x1c`.
///
/// A null slot returns without calls. Otherwise, the task's mode globals can
/// enable a one-argument cleanup helper for the object's `+0xa4` resource;
/// the object release helper always follows. Both calls are represented by
/// checker stand-ins, and the task slot itself is left unchanged.
lf_checker_rt::export!(thiscall, rw_0097EEA0(this: u32) -> () {
    const OBJECT_SLOT: u32 = 0x1C;
    const RESOURCE_OFFSET: u32 = 0xA4;
    const CLEANUP_CALL: u32 = 1;
    const RELEASE_CALL: u32 = 2;
    const CLEANUP_ENABLED_VA: u32 = 0x011F7060;
    const CURRENT_MODE_VA: u32 = 0x012088B4;
    const SKIP_MODE_VA: u32 = 0x01037720;
    const CLEANUP_SENTINEL: u32 = 0xFFFFFFFF;
    const SKIP_MODE: u32 = 0x12;
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    let object = unsafe { read_word(this.wrapping_add(OBJECT_SLOT)) };
    if object == 0 { return; }
    let cleanup_enabled = unsafe { lf_checker_rt::global::<u32>(CLEANUP_ENABLED_VA).read() };
    let current_mode = unsafe { lf_checker_rt::global::<u32>(CURRENT_MODE_VA).read() };
    let skip_mode = unsafe { lf_checker_rt::global::<u32>(SKIP_MODE_VA).read() };
    if cleanup_enabled != 1 && current_mode == CLEANUP_SENTINEL && skip_mode != SKIP_MODE {
        let resource = unsafe { read_word(object.wrapping_add(RESOURCE_OFFSET)) };
        lf_checker_rt::callee_cdecl!(CLEANUP_CALL, u32, resource);
    }
    lf_checker_rt::callee_thiscall!(RELEASE_CALL, u32, object, 0);
});
