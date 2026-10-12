// original: 0x00A61FD0 slot_record_zero

/// Clears the record's leading word, its selected words at +4/+8 and
/// +0x10..+0x18, and +0x20/+0x24. At +0x28 it preserves every bit except the
/// low two, which it clears. Unmentioned fields are left untouched. ECX is
/// the record pointer and the original returns that pointer in EAX.
lf_checker_rt::export!(thiscall, rw_00A61FD0(record: u32) -> u32 {
    const CLEARED_WORDS: [u32; 8] = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24];
    const LOW_FLAGS: u32 = 0x28;
    const PRESERVED_FLAGS: u32 = !0x03;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    for offset in CLEARED_WORDS {
        unsafe { write_u32(record.wrapping_add(offset), 0) };
    }
    let flags = unsafe { read_u32(record.wrapping_add(LOW_FLAGS)) };
    unsafe { write_u32(record.wrapping_add(LOW_FLAGS), flags & PRESERVED_FLAGS) };
    record
});
