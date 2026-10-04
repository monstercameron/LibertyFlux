// original: 0x008a5dc0 audio_fill_three_out_params
/// Resolve three output dwords from pointer-or-default slots.
///
/// Each of the three pointer slots (+0xD0, +0xD8, +0xD4) either points at a
/// dword to copy or is null, in which case a default dword stored in the
/// object (+0xE0, +0xDC, +0xE4) is copied instead. The return value's low
/// byte is 1; its upper three bytes are the third output pointer's, because
/// the original only writes AL over that pointer still sitting in EAX.
export!(thiscall, rw_008a5dc0(
    this: *const u8,
    out0: *mut u32,
    out1: *mut u32,
    out2: *mut u32,
) -> u32 {
    unsafe {
        let p0 = *(this.add(0xD0) as *const u32);
        *out0 = if p0 != 0 {
            *(p0 as *const u32)
        } else {
            *(this.add(0xE0) as *const u32)
        };
        let p1 = *(this.add(0xD8) as *const u32);
        *out1 = if p1 != 0 {
            *(p1 as *const u32)
        } else {
            *(this.add(0xDC) as *const u32)
        };
        let p2 = *(this.add(0xD4) as *const u32);
        *out2 = if p2 != 0 {
            *(p2 as *const u32)
        } else {
            *(this.add(0xE4) as *const u32)
        };
        ((out2 as u32) & 0xFFFF_FF00) | 1
    }
});

