// original: 0x00AD2AE0 audio_release_handle (proposed)

/// Release the cached audio handle through its virtual slot.
///
/// If the handle global is null, returns at once (with the caller's
/// leftover eax, pinned to 0 in the proof). Otherwise calls the destructor
/// in the handle's virtual slot 2 (stdcall/1, the handle itself), clears
/// the global and returns the destructor's answer. Takes no arguments
/// (cdecl/0).
lf_checker_rt::export!(cdecl, rw_00ad2ae0() -> u32 {
    unsafe {
        const HANDLE: u32 = 0x0154E188;
        const DTOR_SLOT: u32 = 8;
        let obj = lf_checker_rt::global::<u32>(HANDLE).read();
        if obj == 0 {
            return 0;
        }
        let vtable = (obj as *const u32).read();
        let target = (vtable.wrapping_add(DTOR_SLOT) as *const u32).read();
        let dtor: extern "stdcall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let r = dtor(obj);
        lf_checker_rt::global::<u32>(HANDLE).write(0);
        r
    }
});
