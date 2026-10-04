// original: 0x00986060 NativeImpl_GET_STATIC_EMITTER_PLAYTIME
/// Original 0x00986060 `NativeImpl_GET_STATIC_EMITTER_PLAYTIME`.
///
/// Returns the dword at +0xb8 of the `idx`-th emitter slot (stride 0xd0 from
/// +0xc8) when the slot is live and its tag byte at +0x3b is 8; else -1.
export!(thiscall, rw_00986060(obj: u32, idx: u32) -> u32 {
    let ent = unsafe {
        (obj.wrapping_add(idx.wrapping_mul(0xd0)).wrapping_add(0xc8) as *const u32).read()
    };
    if ent == 0 {
        return 0xffff_ffff;
    }
    let tag = unsafe { ((ent + 0x3b) as *const u8).read() };
    if tag != 8 {
        return 0xffff_ffff;
    }
    unsafe { ((ent + 0xb8) as *const u32).read() }
});
