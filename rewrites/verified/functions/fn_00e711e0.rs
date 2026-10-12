// original: 0x00E711E0 send_audio_buffer_cleanup_message
/// Gate an audio-buffer cleanup notification with a 16-bit flag and pass the paired 32-bit message pointer when enabled.
lf_checker_rt::export!(cdecl, rw_fn_00E711E0() -> () {
    unsafe {
        const MESSAGE_POINTER: u32 = 0x01168BB0;
        const MESSAGE_ENABLED: u32 = 0x01168BB6;
        const MESSAGE_CALLEE: u32 = 1;

        let enabled = lf_checker_rt::global::<u16>(MESSAGE_ENABLED).read_unaligned();
        if enabled != 0 {
            let message = lf_checker_rt::global::<u32>(MESSAGE_POINTER).read_unaligned();
            let _ = lf_checker_rt::callee_cdecl!(MESSAGE_CALLEE, u32, message);
        }
    }
});
