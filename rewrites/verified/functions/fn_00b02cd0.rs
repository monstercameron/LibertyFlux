// original: 0x00b02cd0 reset_state_words
/// Zero two state words and set four id slots to -1. No inputs, no
/// meaningful return (eax passes through untouched).
export!(cdecl, rw_00b02cd0() -> u32 {
    unsafe {
        *global::<u32>(0x16010dc) = 0;
        *global::<u32>(0x1614c80) = 0;
        *global::<u32>(0x104004c) = 0xFFFF_FFFF;
        *global::<u32>(0x1040050) = 0xFFFF_FFFF;
        *global::<u32>(0x1040054) = 0xFFFF_FFFF;
        *global::<u32>(0x1040058) = 0xFFFF_FFFF;
    }
    0 // unchecked: contract compares no return channel
});
