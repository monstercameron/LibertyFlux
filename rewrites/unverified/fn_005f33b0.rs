// original: 0x005f33b0 CopyChangedPhysicsRecords

/// If the record pointers stored at `this + 0x84` and `this + 0x88` differ,
/// asks the copy helper to copy `count * 64` bytes, where `count` is the
/// unsigned 16-bit field at `this + 0x92`. The helper receives source, target,
/// and byte count as cdecl arguments. The method has no semantic return value.
lf_checker_rt::export!(thiscall, rw_005f33b0(this: u32) -> u32 {
    const TARGET_RECORD: u32 = 0x84;
    const SOURCE_RECORD: u32 = 0x88;
    const RECORD_COUNT: u32 = 0x92;
    const RECORD_STRIDE: u32 = 0x40;

    let source = unsafe { (this.wrapping_add(SOURCE_RECORD) as *const u32).read_unaligned() };
    let target = unsafe { (this.wrapping_add(TARGET_RECORD) as *const u32).read_unaligned() };
    if source != target {
        let count = unsafe { (this.wrapping_add(RECORD_COUNT) as *const u16).read_unaligned() } as u32;
        let byte_count = count.wrapping_mul(RECORD_STRIDE);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, source, target, byte_count);
    }
    0
});
