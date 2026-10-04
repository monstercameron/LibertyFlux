// original: 0x00ab8270 record_publish_all
/// Publishes the seven tag bytes of the record through the tag sink, each
/// with its fixed slot index. Returns the sink's last answer.

export!(thiscall, rw_00ab8270(this: u32, rec: *const u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this, 0, *rec.add(0x5C) as u32);
        callee_thiscall!(1, u32, this, 1, *rec.add(0x5D) as u32);
        callee_thiscall!(1, u32, this, 2, *rec.add(0x5E) as u32);
        callee_thiscall!(1, u32, this, 3, *rec.add(0x5F) as u32);
        callee_thiscall!(1, u32, this, 5, *rec.add(0x61) as u32);
        callee_thiscall!(1, u32, this, 7, *rec.add(0x63) as u32);
        callee_thiscall!(1, u32, this, 0xA, *rec.add(0x66) as u32)
    }
});
