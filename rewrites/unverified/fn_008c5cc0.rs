// original: 0x008C5CC0 n:/text
/// Select the requested related record from the shared root's table. A
/// nonpositive signed count returns the static default; positive counts walk
/// from the first matching row and may fall back to the root's manager record.
lf_checker_rt::export!(stdcall, rw_008c5cc0(requested: u32) -> u32 {
    const ROOT_FILE_VA: u32 = 0x1BB5_628;
    const DEFAULT_FILE_VA: u32 = 0x00E7_F938;
    const RECORD_STRIDE: u32 = 0x88;
    unsafe {
        let root = lf_checker_rt::global::<u32>(ROOT_FILE_VA).read_unaligned();
        let table_count = (root.wrapping_add(4) as *const u16).read_unaligned() as u32;
        let table = (root as *const u32).read_unaligned();
        let mut current = root.wrapping_add(0x690);
        for index in 0..table_count {
            let candidate = table.wrapping_add(0x80 + index * RECORD_STRIDE);
            if (candidate as *const u32).read_unaligned() == 6 {
                current = table.wrapping_add(index * RECORD_STRIDE);
                break;
            }
        }
        let mut remaining = requested as i32;
        if remaining <= 0 { return lf_checker_rt::relocated(DEFAULT_FILE_VA); }
        loop {
            remaining -= 1;
            if remaining == 0 { return current; }
            let mut next = (current.wrapping_add(0x84) as *const u32)
                .read_unaligned().wrapping_add(1);
            if next >= table_count {
                current = root.wrapping_add(0x690);
            } else {
                let key = (current.wrapping_add(0x80) as *const u32).read_unaligned();
                let mut matched = false;
                while next < table_count {
                    let candidate = table.wrapping_add(next * RECORD_STRIDE + 0x80);
                    if (candidate as *const u32).read_unaligned() == key {
                        current = table.wrapping_add(next * RECORD_STRIDE);
                        matched = true;
                        break;
                    }
                    next = next.wrapping_add(1);
                }
                if !matched { current = root.wrapping_add(0x690); }
            }
        }
    }
});
