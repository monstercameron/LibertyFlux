// original: 0x00A61370 copy_record20

/// Copies five 32-bit words from object offsets 0x50 through 0x60 into the
/// destination in ascending order. The two middle words contain scalar float
/// bits and are transferred without arithmetic. ECX is the source object and
/// one thiscall stack argument points to the destination. EAX is not a
/// semantic result; the copied 20 bytes are the observable effect.
lf_checker_rt::export!(thiscall, rw_00A61370(this: u32, destination: u32) -> u32 {
    const FIRST_FIELD: u32 = 0x50;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    let mut index = 0u32;
    while index < 5 {
        let value = unsafe { read_u32(this.wrapping_add(FIRST_FIELD + index * 4)) };
        unsafe { write_u32(destination.wrapping_add(index * 4), value) };
        index += 1;
    }
    0
});
