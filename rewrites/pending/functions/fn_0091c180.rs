// original: 0x0091C180 INPUT_KB_PHONE_ACCEPT
/// Resolve the phone accept or cancel binding through the binding lookup.
///
/// When the first flag byte is nonzero it works on the accept binding and
/// defaults to `0x11C`, otherwise on the cancel binding defaulting to
/// `0x11D`: if the second flag byte is nonzero, or the context pointer is
/// null, the default is returned at once, else the binding name and context
/// go to the lookup helper and its answer is returned unless it is `0xFF`,
/// in which case the default is returned. Only the low bytes of the two
/// flag words are read.
export!(cdecl, rw_0091c180(flag0: u32, flag1: u32, ctx: u32) -> u32 {
    unsafe {
        if (flag0 & 0xFF) != 0 {
            if (flag1 & 0xFF) != 0 || ctx == 0 {
                return 0x11C;
            }
            let r: u32 = callee_cdecl!(1, u32, relocated(0xE85D94), ctx, 0);
            if r == 0xFF { 0x11C } else { r }
        } else {
            if (flag1 & 0xFF) != 0 || ctx == 0 {
                return 0x11D;
            }
            let r: u32 = callee_cdecl!(1, u32, relocated(0xE85DAC), ctx, 0);
            if r == 0xFF { 0x11D } else { r }
        }
    }
});
