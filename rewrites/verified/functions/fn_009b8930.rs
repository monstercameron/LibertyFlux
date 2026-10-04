// original: 0x009b8930 HintFlag_SetIfLowByteZero
/// Sets a global hint flag to 1 when the low byte of the argument is zero,
/// else clears it to 0. Returns nothing meaningful (the original leaves its
/// entry EAX untouched, so the return channel is not compared).
export!(stdcall, rw_009b8930(a: u32) -> u32 {
    unsafe {
        *global::<u8>(0x0128E3F9) = ((a & 0xFF) == 0) as u8;
        0
    }
});
