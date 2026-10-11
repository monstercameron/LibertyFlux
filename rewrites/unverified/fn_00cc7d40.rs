// original: 0x00CC7D40 fragInstGta::vf34

/// Updates the fragment's three-word global record for each entry reported by
/// the physics manager. It returns zero when receiver flags `0x0400` are set,
/// or when the referenced record's byte at `+0x24` has any of bits `0x1c` set.
/// Otherwise, if the receiver bitmap at `+0xa4` is zero, it forwards all three
/// stack arguments to the helper. With a nonzero bitmap, it sets bit zero in a
/// global initialization word, then visits the manager's byte-sized count at
/// `+0x1f3`: set bitmap bits zero the corresponding three global words, while
/// clear bits copy the three words from source records spaced 16 bytes apart.
/// The final helper receives the global table base and the last two stack
/// arguments. Manager lookups and helper effects are scripted callees.
lf_checker_rt::export!(thiscall, rw_00cc7d40(
    this: u32,
    source: u32,
    arg1: u32,
    arg2: u32,
) -> u8 {
    const MANAGER_LOOKUP: u32 = 1;
    const RECORD_HELPER: u32 = 2;
    const RECEIVER_FLAGS: u32 = 0x0a;
    const REFERENCED_RECORD: u32 = 0x0c;
    const RECORD_FLAGS: u32 = 0x24;
    const RECEIVER_BITMAP: u32 = 0xa4;
    const MANAGER_COUNT: u32 = 0x1f3;
    const FLAG_MASK: u16 = 0x0400;
    const RECORD_FLAG_MASK: u8 = 0x1c;
    const INITIALIZED_FLAG: u32 = 0x0171_c930;
    const TABLE_BASE: u32 = 0x0171_c130;
    const TABLE_STRIDE: u32 = 0x10;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }
    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }
    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let receiver_flags = read_u16(this.wrapping_add(RECEIVER_FLAGS));
        if receiver_flags & FLAG_MASK != 0 {
            return 0;
        }
        let record = read_u32(this.wrapping_add(REFERENCED_RECORD));
        if record != 0 && read_u8(record.wrapping_add(RECORD_FLAGS)) & RECORD_FLAG_MASK != 0 {
            return 0;
        }

        let bitmap_address = this.wrapping_add(RECEIVER_BITMAP);
        let bitmap = read_u32(bitmap_address);
        if bitmap == 0 {
            return lf_checker_rt::callee_thiscall!(RECORD_HELPER, u32, this, source, arg1, arg2) as u8;
        }

        let initialized = lf_checker_rt::global::<u32>(INITIALIZED_FLAG).read_unaligned();
        if initialized & 1 == 0 {
            lf_checker_rt::global::<u32>(INITIALIZED_FLAG).write_unaligned(initialized | 1);
        }

        let source_data = source;
        let mut manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, this);
        let mut count = read_u8(manager.wrapping_add(MANAGER_COUNT)) as u32;
        let mut index = 0u32;
        while index < count {
            let entry_address = TABLE_BASE.wrapping_add(index.wrapping_mul(TABLE_STRIDE));
            let source_address = source_data.wrapping_add(index.wrapping_mul(TABLE_STRIDE));
            if bitmap & (1u32 << (index & 31)) != 0 {
                write_u32(lf_checker_rt::relocated(entry_address), 0);
                write_u32(lf_checker_rt::relocated(entry_address.wrapping_add(4)), 0);
                write_u32(lf_checker_rt::relocated(entry_address.wrapping_add(8)), 0);
            } else {
                write_u32(lf_checker_rt::relocated(entry_address), read_u32(source_address));
                write_u32(lf_checker_rt::relocated(entry_address.wrapping_add(4)), read_u32(source_address.wrapping_add(4)));
                write_u32(lf_checker_rt::relocated(entry_address.wrapping_add(8)), read_u32(source_address.wrapping_add(8)));
            }
            index += 1;
            manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, this);
            count = read_u8(manager.wrapping_add(MANAGER_COUNT)) as u32;
        }

        let table = lf_checker_rt::relocated(TABLE_BASE);
        lf_checker_rt::callee_thiscall!(RECORD_HELPER, u32, this, table, arg1, arg2) as u8
    }
});
