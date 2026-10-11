// original: 0x00CC78C0 update_physics_feature_bitset_from_record (proposed)

/// Updates the receiver's 32-bit feature bitmap at `+0xa4` from a manager
/// record. The row index selects a metadata pointer from the manager table at
/// `+0xd4`; its byte at `+0x0c` selects a record pointer from the table at
/// `+0xd0`. Each record clears `count` bits beginning at its byte-sized start
/// bit, wrapping shifts modulo 32, then appends `list_count` consecutive record
/// codes beginning at its byte-sized base. Pending codes are processed in
/// last-in-first-out order, with the worklist capped at 256 entries. The
/// manager lookup and stack-cookie check are scripted callees; the function has
/// one thiscall stack argument and no defined return value.
lf_checker_rt::export!(thiscall, rw_00cc78c0(this: u32, row_index: u32) -> () {
    const MANAGER_LOOKUP: u32 = 1;
    const STACK_COOKIE_CHECK: u32 = 2;
    const MANAGER_RECORDS: u32 = 0xd0;
    const MANAGER_ROWS: u32 = 0xd4;
    const RECEIVER_BITMAP: u32 = 0xa4;
    const ROW_CODE: u32 = 0x0c;
    const RECORD_BASE: u32 = 0x10c;
    const RECORD_START_BIT: u32 = 0x10e;
    const RECORD_CLEAR_COUNT: u32 = 0x10f;
    const RECORD_LIST_COUNT: u32 = 0x110;
    const WORKLIST_CAPACITY: usize = 256;
    const STACK_COOKIE: u32 = 0x0105_7fb4;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
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
        let manager = lf_checker_rt::callee_thiscall!(MANAGER_LOOKUP, u32, this);
        let records = read_u32(manager.wrapping_add(MANAGER_RECORDS));
        let rows = read_u32(manager.wrapping_add(MANAGER_ROWS));
        let row = read_u32(rows.wrapping_add(row_index.wrapping_mul(4)));
        let code = read_u8(row.wrapping_add(ROW_CODE)) as u32;
        let mut record = read_u32(records.wrapping_add(code.wrapping_mul(4)));
        let mut pending = [0u32; WORKLIST_CAPACITY];
        let mut pending_len = 0usize;

        loop {
            let clear_count = read_u8(record.wrapping_add(RECORD_CLEAR_COUNT)) as u32;
            let first_bit = read_u8(record.wrapping_add(RECORD_START_BIT)) as u32;
            for bit_index in 0..clear_count {
                let bit = first_bit.wrapping_add(bit_index) & 31;
                let bitmap_address = this.wrapping_add(RECEIVER_BITMAP);
                let bitmap = read_u32(bitmap_address) & !(1u32 << bit);
                write_u32(bitmap_address, bitmap);
            }

            let base_code = read_u8(record.wrapping_add(RECORD_BASE)) as u32;
            let list_count = read_u8(record.wrapping_add(RECORD_LIST_COUNT)) as usize;
            let mut child = 0usize;
            while child < list_count && pending_len < WORKLIST_CAPACITY {
                pending[pending_len] = base_code.wrapping_add(child as u32);
                pending_len += 1;
                child += 1;
            }

            if pending_len == 0 {
                break;
            }
            let next_code = pending[pending_len - 1];
            pending_len -= 1;
            record = read_u32(records.wrapping_add(next_code.wrapping_mul(4)));
        }

        let cookie = read_u32(lf_checker_rt::global::<u32>(STACK_COOKIE) as u32);
        let _ = lf_checker_rt::callee_thiscall!(STACK_COOKIE_CHECK, u32, cookie);
    }
});
