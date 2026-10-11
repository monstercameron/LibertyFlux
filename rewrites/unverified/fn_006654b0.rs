// original: 0x006654B0 network_read_keyframe_block

/// Read a packed network keyframe record. The fastcall `object` supplies the
/// parser service and `record` is a writable buffer descriptor: buffer at
/// `+0`, output base at `+4`, capacity at `+8`, cursor at `+0xc`, two element
/// counts at `+0x10` and `+0x14`, flags at `+0x18`, and the element kind at
/// `+0x24`. The method reads three header fields, rejects either count above
/// 512 using unsigned comparisons, copies each nonempty element range, writes
/// one element at the cursor, advances the cursor and high-water count, then
/// optionally parses one variable-width element. Parser and copy calls are
/// made in their original order; the result is a low-byte success value.
lf_checker_rt::export!(fastcall, rw_006654B0(object: u32, record: u32) -> u32 {
    unsafe {
        const CHECK_HEADER: u32 = 1;
        const READ_RECORD: u32 = 2;
        const READ_KIND: u32 = 3;
        const READ_PRIMARY_COUNT: u32 = 4;
        const READ_SECONDARY_COUNT: u32 = 5;
        const COPY_PRIMARY: u32 = 6;
        const COPY_SECONDARY: u32 = 7;
        const WRITE_ELEMENT: u32 = 8;
        const READ_VARIABLE_COUNT: u32 = 9;
        const READ_VARIABLE_ELEMENT: u32 = 10;
        const RECORD_BUFFER: u32 = 0;
        const RECORD_OUTPUT: u32 = 4;
        const RECORD_CAPACITY: u32 = 8;
        const RECORD_CURSOR: u32 = 0x0c;
        const RECORD_PRIMARY_COUNT: u32 = 0x10;
        const RECORD_SECONDARY_COUNT: u32 = 0x14;
        const RECORD_FLAGS: u32 = 0x18;
        const RECORD_PRIMARY_DATA: u32 = 0x18;
        const RECORD_SECONDARY_DATA: u32 = 0x218;
        const RECORD_KIND: u32 = 0x24;
        const COUNT_LIMIT: u32 = 0x200;

        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        unsafe fn rd8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        if lf_checker_rt::callee_thiscall!(CHECK_HEADER, u32, object) as u8 == 0 {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(
            READ_RECORD,
            u32,
            object,
            record.wrapping_add(RECORD_CAPACITY),
            0,
        ) as u8 == 0
        {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(
            READ_KIND,
            u32,
            object,
            record.wrapping_add(RECORD_CURSOR),
            3,
        ) as u8 == 0
        {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(
            READ_PRIMARY_COUNT,
            u32,
            object,
            record.wrapping_add(RECORD_PRIMARY_COUNT),
            10,
        ) as u8 == 0
        {
            return 0;
        }
        let primary_count = rd32(record.wrapping_add(RECORD_PRIMARY_COUNT));
        if primary_count > COUNT_LIMIT {
            return 0;
        }
        if lf_checker_rt::callee_thiscall!(
            READ_SECONDARY_COUNT,
            u32,
            object,
            record.wrapping_add(RECORD_SECONDARY_COUNT),
            10,
        ) as u8 == 0
        {
            return 0;
        }
        let secondary_count = rd32(record.wrapping_add(RECORD_SECONDARY_COUNT));
        if secondary_count > COUNT_LIMIT {
            return 0;
        }
        if primary_count != 0
            && lf_checker_rt::callee_thiscall!(
                COPY_PRIMARY,
                u32,
                object,
                record.wrapping_add(RECORD_PRIMARY_DATA),
                primary_count.wrapping_shl(3),
                0,
            ) as u8
                == 0
        {
            return 0;
        }
        if secondary_count != 0
            && lf_checker_rt::callee_thiscall!(
                COPY_SECONDARY,
                u32,
                object,
                record.wrapping_add(RECORD_SECONDARY_DATA),
                secondary_count.wrapping_shl(3),
                0,
            ) as u8
                == 0
        {
            return 0;
        }
        if primary_count | secondary_count == 0 {
            return 0;
        }
        if rd8(record.wrapping_add(RECORD_FLAGS)) & 1 != 0 {
            return 0;
        }
        let cursor = rd32(record.wrapping_add(RECORD_CURSOR));
        let next_cursor = cursor.wrapping_add(1);
        if (next_cursor as i32) > (rd32(record.wrapping_add(RECORD_CAPACITY)) as i32) {
            return 0;
        }
        let element_kind = u32::from(rd8(record.wrapping_add(RECORD_KIND)));
        let destination = rd32(record.wrapping_add(RECORD_OUTPUT)).wrapping_add(cursor);
        let buffer = rd32(record.wrapping_add(RECORD_BUFFER));
        let _ = lf_checker_rt::callee_cdecl!(WRITE_ELEMENT, u32, buffer, 1, element_kind, destination);
        wr32(record.wrapping_add(RECORD_CURSOR), next_cursor);
        if (next_cursor as i32) > (primary_count as i32) {
            wr32(record.wrapping_add(RECORD_PRIMARY_COUNT), next_cursor);
        }
        if element_kind == 0 {
            return 1;
        }
        let _ = lf_checker_rt::callee_thiscall!(
            READ_VARIABLE_COUNT,
            u32,
            record,
            record.wrapping_add(RECORD_PRIMARY_COUNT),
            1,
        );
        let variable_count = rd32(record.wrapping_add(RECORD_PRIMARY_COUNT)) as i32;
        if variable_count > 1 {
            return 0;
        }
        let mut index = 0i32;
        let mut item = record.wrapping_add(RECORD_PRIMARY_DATA);
        while index < variable_count {
            if lf_checker_rt::callee_thiscall!(READ_VARIABLE_ELEMENT, u32, record, item) as u8 == 0 {
                return 0;
            }
            index = index.wrapping_add(1);
            item = item.wrapping_add(8);
        }
        1
    }
});
