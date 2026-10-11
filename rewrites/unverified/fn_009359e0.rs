// original: 0x009359E0 write_network_record_batch

/// Serialize a network record batch through the stream writer. The receiver
/// stores the record array pointer at `+0x144` and a 16-bit record count at
/// `+0x148`; each record is `0x22c` bytes, with a one-byte key at `+0x228` and
/// a 64-byte payload beginning at `+4`. A helper supplies the two-word
/// summary header. The routine writes that header, each record key and
/// payload, then zero key/payload pairs until the batch reaches 64 entries.
/// Counts above 64 are serialized without padding. The function returns one
/// in EAX and uses the x86 thiscall convention.
lf_checker_rt::export!(thiscall, rw_009359e0(this: u32) -> u32 {
    unsafe {
        const RECORDS_POINTER: u32 = 0x144;
        const RECORD_COUNT: u32 = 0x148;
        const RECORD_STRIDE: u32 = 0x22c;
        const RECORD_KEY: u32 = 0x228;
        const RECORD_PAYLOAD: u32 = 4;
        const SUMMARY_BYTES: u32 = 8;
        const RECORD_PAYLOAD_BYTES: u32 = 0x40;
        const BATCH_LIMIT: u32 = 0x40;

        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let summary_low_high = lf_checker_rt::callee_thiscall!(1, u64, this);
        let mut summary = [summary_low_high as u32, (summary_low_high >> 32) as u32];
        let _ = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            summary.as_mut_ptr() as u32,
            SUMMARY_BYTES
        );

        let records = read_u32(this.wrapping_add(RECORDS_POINTER));
        let record_count = u32::from(read_u16(this.wrapping_add(RECORD_COUNT)));
        for index in 0..record_count {
            let record = records.wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let key = record.wrapping_add(RECORD_KEY);
            let payload = record.wrapping_add(RECORD_PAYLOAD);
            let _ = lf_checker_rt::callee_cdecl!(3, u32, key, 1);
            let _ = lf_checker_rt::callee_cdecl!(4, u32, payload, RECORD_PAYLOAD_BYTES);
        }

        if record_count < BATCH_LIMIT {
            let zero_key = 0u8;
            let zero_payload = [0u8; RECORD_PAYLOAD_BYTES as usize];
            for _ in record_count..BATCH_LIMIT {
                let _ = lf_checker_rt::callee_cdecl!(
                    5,
                    u32,
                    core::ptr::addr_of!(zero_key) as u32,
                    1
                );
                let _ = lf_checker_rt::callee_cdecl!(
                    6,
                    u32,
                    zero_payload.as_ptr() as u32,
                    RECORD_PAYLOAD_BYTES
                );
            }
        }

        1
    }
});
