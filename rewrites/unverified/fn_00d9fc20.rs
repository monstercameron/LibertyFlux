// original: 0x00D9FC20 stream_object_active
/// Looks up the object state associated with the receiver's object key and tests its flags and counter.
lf_checker_rt::export!(thiscall, rw_s675_00d9fc20(this: u32) -> u32 {
    const OBJECT_OFFSET: u32 = 0x18;
    const FLAGS_OFFSET: u32 = 0x20;
    let state: u32 = lf_checker_rt::callee_cdecl!(1, u32, this.wrapping_add(OBJECT_OFFSET));
    let flags = unsafe { (state.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned() };
    (flags >> 9) & 1
});
