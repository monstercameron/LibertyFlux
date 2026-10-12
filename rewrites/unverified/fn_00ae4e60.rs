// original: 0x00AE4E60 T_CB_Generic_5Args<void(*)(int, RenderListId, CRenderer::RenderBucketTypes, eRenderMode, int), int, RenderListId, CRenderer::RenderBucketTypes, eRenderMode, int>::invoke

/// Invoke the stored cdecl callback with the five values stored in order in its wrapper object.
/// The callback pointer is stored at object offset `+0x08` and the
/// object is passed in ECX. The callback uses cdecl and its return
/// value is returned unchanged in EAX. The contract plants its scripted
/// stand-in at that callback slot; the runtime call names the same stand-in.
lf_checker_rt::export!(thiscall, rw_00ae4e60(this: u32) -> u32 {
    const CALLBACK_ID: u32 = 1;
    let first_value = unsafe { (this.wrapping_add(0xC) as *const u32).read_unaligned() };
    let second_value = unsafe { (this.wrapping_add(0x10) as *const u32).read_unaligned() };
    let third_value = unsafe { (this.wrapping_add(0x14) as *const u32).read_unaligned() };
    let fourth_value = unsafe { (this.wrapping_add(0x18) as *const u32).read_unaligned() };
    let fifth_value = unsafe { (this.wrapping_add(0x1C) as *const u32).read_unaligned() };
    lf_checker_rt::callee_cdecl!(CALLBACK_ID, u32,
        first_value,
        second_value,
        third_value,
        fourth_value,
        fifth_value,
    )
});
