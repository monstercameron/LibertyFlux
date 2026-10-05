// original: 0x00bebd10 zero_fields_4
/// Zero four dwords of a task-state block starting at offset 8.
///
/// `this` points to at least 0x18 bytes. Zeroes the dwords at offsets 0x08,
/// 0x0c, 0x10 and 0x14; bytes 0x00-0x07 are never touched. Returns nothing
/// (EAX keeps the caller's value). Thiscall, no stack arguments.
export!(thiscall, rw_00bebd10(this: u32) -> u32 {
    unsafe {
        for off in [0x08u32, 0x0c, 0x10, 0x14] {
            ((this + off) as *mut u32).write_unaligned(0);
        }
        0
    }
});
