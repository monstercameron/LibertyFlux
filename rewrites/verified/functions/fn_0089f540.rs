// original: 0x0089F540 aud_resolve_if_active
// ---------------------------------------------------------------------------
// 0x0089F540: resolve the object keyed by this sound's id word (+0x3C) and
// return it only when its active flag (+0x48) is set, else null.
// ---------------------------------------------------------------------------
export!(thiscall, rw_0089F540(this_ptr: u32) -> u32 {
    let key = unsafe { *((this_ptr.wrapping_add(0x3C)) as *const i16) } as u32;
    let obj = callee_cdecl!(1, u32, key);
    if obj == 0 {
        return 0;
    }
    let active = unsafe { *((obj.wrapping_add(0x48)) as *const u8) };
    if active != 0 { obj } else { 0 }
});
