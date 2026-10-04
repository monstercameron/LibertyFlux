// original: 0x008d7980 record_fill
/// Fill a record from two source pairs, a global tag and two scalars.
///
/// Sets the flag byte at +0x1C, copies the global tag word to +0x10, the two
/// scalar arguments to +0x14/+0x18, and two dwords from each source pointer
/// to +0/+4 and +8/+0xC. Returns the last word copied (left in EAX).
export!(thiscall, rw_008d7980(this_: *mut u32, a: *const u32, b: *const u32, c: u32, d: u32) -> u32 {
    unsafe {
        *(this_ as *mut u8).byte_add(0x1C) = 1;
        *this_.byte_add(0x10) = *global::<u32>(0x011735D4);
        *this_.byte_add(0x14) = c;
        *this_.byte_add(0x18) = d;
        *this_ = *a;
        *this_.byte_add(0x04) = *a.add(1);
        *this_.byte_add(0x08) = *b;
        let last = *b.add(1);
        *this_.byte_add(0x0C) = last;
        last
    }
});
