// original: 0x00e045d5 guarded_teardown
/// Tears down an object when enabled and flag bit 0x1000 is present.
///
/// Does nothing when `enable` is zero or the flag word at offset 0xC
/// lacks bit 0x1000; otherwise invokes the teardown callee, clears bits
/// 0x1100 and zeroes the link words. Returns 0 on the teardown path;
/// other paths leave EAX untouched, so the return channel is unchecked.
export!(cdecl, rw_00e045d5(enable: u32, obj: u32) -> u32 {
    unsafe {
        if enable != 0 {
            let flags = (obj + 0xC) as *mut u32;
            if *flags & 0x1000 != 0 {
                callee_cdecl!(1, u32, obj);
                *flags &= 0xFFFFEEFF;
                *((obj + 0x18) as *mut u32) = 0;
                *(obj as *mut u32) = 0;
                *((obj + 8) as *mut u32) = 0;
            }
        }
        0
    }
});
