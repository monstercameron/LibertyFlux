// original: 0x009a4830 audio_submit_armed_request
/// Original 0x009a4830 (unnamed): submit an armed audio request.
///
/// Returns at once when the armed flag at +0x8a, the pending flag at +0x8d,
/// or the global block byte is set. Otherwise builds a request frame on the
/// stack through the frame helper, tags it, and submits it with the global
/// channel to the request sink on the shared queue object. The early exits
/// leave incoming registers behind, so no return value is compared.
export!(thiscall, rw_009a4830(this_: u32) -> u32 {
    if unsafe { ((this_ + 0x8a) as *const u8).read() } != 0 {
        return 0;
    }
    if unsafe { ((this_ + 0x8d) as *const u8).read() } != 0 {
        return 0;
    }
    if unsafe { (relocated(0x012845C9) as *const u8).read() } != 0 {
        return 0;
    }
    let mut buf = [0u32; 18];
    callee_thiscall!(1, u32, buf.as_mut_ptr() as u32);
    buf[14] = 2;
    let ch = unsafe { (relocated(0x01038E60) as *const u32).read() };
    callee_thiscall!(
        2,
        u32,
        relocated(0x012845D0),
        ch,
        buf.as_ptr() as u32,
        0xffff_ffff,
        0,
        0
    )
});
