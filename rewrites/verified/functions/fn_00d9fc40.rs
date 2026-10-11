// original: 0x00D9FC40 stream_object_ready
/// Looks up the object state associated with the receiver's object key and tests its flags and counter.
lf_checker_rt::export!(thiscall, rw_s675_00d9fc40(this: u32) -> u32 {
    const OBJECT_OFFSET: u32 = 0x18;
    const FLAGS_OFFSET: u32 = 0x20;
    const COUNT_OFFSET: u32 = 0x8C;
    let object_key = unsafe { (this.wrapping_add(OBJECT_OFFSET) as *const u32).read_unaligned() };
    let state: u32 = lf_checker_rt::callee_cdecl!(1, u32, object_key);
    let flags = unsafe { (state.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned() };
    let count = unsafe { (state.wrapping_add(COUNT_OFFSET) as *const i32).read_unaligned() };
    (((flags >> 5) & 1) != 0 && count < 200) as u32
});
