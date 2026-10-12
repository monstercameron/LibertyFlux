// original: 0x00A60F60 copy_record64

/// Copies four consecutive 16-byte groups from object offset +0x10 to a
/// destination pointer. Within each group, the first three words are loaded
/// before their stores and the fourth is loaded afterward, preserving the
/// original ordering for overlapping regions. Float fields are copied as raw
/// words. ECX is the source object and one thiscall stack argument is the
/// destination; EAX is not a semantic result.
lf_checker_rt::export!(thiscall, rw_00A60F60(this: u32, destination: u32) -> u32 {
    const FIRST_GROUP: u32 = 0x10;
    const GROUPS_BEFORE_LAST: [u32; 3] = [0x10, 0x20, 0x30];
    const LAST_GROUP: u32 = 0x40;
    const LAST_WORD: u32 = 0x4c;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    for source_offset in GROUPS_BEFORE_LAST {
        let first = unsafe { read_u32(this.wrapping_add(source_offset)) };
        let second = unsafe { read_u32(this.wrapping_add(source_offset + 4)) };
        let third = unsafe { read_u32(this.wrapping_add(source_offset + 8)) };
        let destination_offset = source_offset - FIRST_GROUP;
        unsafe {
            write_u32(destination.wrapping_add(destination_offset), first);
            write_u32(destination.wrapping_add(destination_offset + 4), second);
            write_u32(destination.wrapping_add(destination_offset + 8), third);
        }
        let fourth = unsafe { read_u32(this.wrapping_add(source_offset + 12)) };
        unsafe { write_u32(destination.wrapping_add(destination_offset + 12), fourth) };
    }

    let first = unsafe { read_u32(this.wrapping_add(LAST_GROUP)) };
    let second = unsafe { read_u32(this.wrapping_add(LAST_GROUP + 4)) };
    let third = unsafe { read_u32(this.wrapping_add(LAST_GROUP + 8)) };
    let destination_offset = LAST_GROUP - FIRST_GROUP;
    unsafe {
        write_u32(destination.wrapping_add(destination_offset), first);
        write_u32(destination.wrapping_add(destination_offset + 4), second);
        write_u32(destination.wrapping_add(destination_offset + 8), third);
    }
    let fourth = unsafe { read_u32(this.wrapping_add(LAST_WORD)) };
    unsafe { write_u32(destination.wrapping_add(destination_offset + 12), fourth) };
    0
});
