// original: 0x009C2840 stream_active_asset_cleanup

/// Visit each active streaming record up to the signed global count. For a
/// non-null record, query its resource state; when the state is usable, look
/// up its event record and dispatch the record to one of the two cleanup
/// helpers according to its byte status. The routine has no declared return
/// value; its observable outputs are the ordered helper calls.
lf_checker_rt::export!(cdecl, rw_009c2840() -> () {
    unsafe {
        const ACTIVE_COUNT: u32 = 0x0129_571c;
        const ACTIVE_RECORDS: u32 = 0x0129_5268;
        const RECORD_VECTOR: u32 = 0x10;
        const RECORD_POSITION: u32 = 0x30;
        const EVENT_STATUS: u32 = 0x22a;
        const CLEANUP_STATUS_A: u8 = 3;
        const CLEANUP_STATUS_B: u8 = 4;

        let count = lf_checker_rt::global::<i32>(ACTIVE_COUNT).read_unaligned();
        let mut index = 0i32;
        while index < count {
            let slot = ACTIVE_RECORDS.wrapping_add((index as u32).wrapping_mul(4));
            let record = lf_checker_rt::global::<u32>(slot).read_unaligned();
            if record != 0 {
                let vector_address = record.wrapping_add(RECORD_VECTOR);
                let resource_state = lf_checker_rt::callee_cdecl!(1, u32, vector_address);
                if resource_state != u32::MAX {
                    let position_bits =
                        (record.wrapping_add(RECORD_POSITION) as usize as *const u32)
                            .read_unaligned();
                    let event = lf_checker_rt::callee_cdecl!(
                        2,
                        u32,
                        record,
                        position_bits,
                        resource_state,
                    );
                    if event != 0 {
                        let status =
                            (event.wrapping_add(EVENT_STATUS) as usize as *const u8).read_unaligned();
                        if status == CLEANUP_STATUS_A || status == CLEANUP_STATUS_B {
                            let _ = lf_checker_rt::callee_cdecl!(4, u32, event);
                        } else {
                            let _ = lf_checker_rt::callee_cdecl!(3, u32, event, 0u32);
                        }
                    }
                }
            }
            index += 1;
        }
    }
});
