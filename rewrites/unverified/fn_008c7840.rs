// original: 0x008C7840 stream_channel_openclose
/// Open or close streaming channel `ch` while the busy flags are raised.
///
/// Raises the busy pair, then calls the open callee when `create` is
/// nonzero and the close callee (with mode 0) when it is zero. Always
/// lowers the busy pair again and clears the channel's status byte at
/// `ch + 8`. Returns nothing. Original: cdecl, two stack words.
lf_checker_rt::export!(cdecl, rw_008c7840(create: u32, ch: u32) -> u32 {
    unsafe {
        const OPEN_CALLEE: u32 = 1;
        const CLOSE_CALLEE: u32 = 2;
        const STATUS: u32 = 8;
        *lf_checker_rt::global::<u8>(0x116d27d) = 1;
        *lf_checker_rt::global::<u8>(0x116d27f) = 0;
        if (create as u8) != 0 {
            lf_checker_rt::callee_cdecl!(OPEN_CALLEE, u32, ch);
        } else {
            lf_checker_rt::callee_cdecl!(CLOSE_CALLEE, u32, ch, 0);
        }
        *lf_checker_rt::global::<u8>(0x116d27d) = 0;
        *lf_checker_rt::global::<u8>(0x116d27f) = 0;
        ((ch + STATUS) as *mut u8).write(0);
        0
    }
});
