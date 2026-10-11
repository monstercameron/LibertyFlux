// original: 0x00876AB0 crmt_frame_request_initialize

/// Initialize the frame request's embedded source and store its frame index
/// and one-byte mode. The embedded source is at +0x04; its setup hook is
/// called through the shared data slot before the request vtable is installed.
/// The second stack word contributes only its low byte. Return `this`.
lf_checker_rt::export!(thiscall, rw_00876ab0(this: u32, frame_index: u32, mode_word: u32) -> u32 {
    const SUBOBJECT: u32 = 0x04;
    const OBSERVER_VTABLE: u32 = 0x00fe7fb4;
    const SOURCE_VTABLE: u32 = 0x00fe7fc8;
    const FRAME_VTABLE: u32 = 0x00fe8224;
    const SETUP_HOOK_VA: u32 = 0x00fe7fb8;

    unsafe {
        let source = this + SUBOBJECT;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(SOURCE_VTABLE));
        ((source + 4) as *mut u32).write_unaligned(0);
        (source as *mut u32).write_unaligned(lf_checker_rt::relocated(OBSERVER_VTABLE));
        ((source + 8) as *mut u32).write_unaligned(0);
        ((source + 0x0c) as *mut u32).write_unaligned(0);

        let hook_target = lf_checker_rt::global::<u32>(SETUP_HOOK_VA).read_unaligned();
        let setup: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(hook_target as usize);
        let _ = setup(source);

        ((this + 0x14) as *mut u32).write_unaligned(frame_index);
        ((this + 0x18) as *mut u8).write(mode_word as u8);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(FRAME_VTABLE));
        this
    }
});
