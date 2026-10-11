// original: 0x00665400 network_scan_accepted_records

/// Scan the object's record-pointer table and run the first acceptance
/// callback for each entry. The unsigned count at `this + 0x2ea4` bounds the
/// table at `this + 0x2e24`; the callback service pointer is at `this + 0x24`.
/// Accepted records are decoded into two temporary words. A successful decode
/// is committed through the handler object at `this + 0xc6c`, with flags `0, 0`.
/// The method is 32-bit thiscall with two stack arguments and no meaningful
/// return value.
lf_checker_rt::export!(thiscall, rw_00665400(this: u32, source: u32, _unused: u32) -> u32 {
    unsafe {
        const SERVICE_PTR: u32 = 0x24;
        const RECORDS: u32 = 0x2e24;
        const RECORD_COUNT: u32 = 0x2ea4;
        const HANDLER: u32 = 0xc6c;
        const STACK_COOKIE_VA: u32 = 0x0105_7fb4;
        const ACCEPT_RECORD: u32 = 1;
        const DECODE_RECORD: u32 = 2;
        const COMMIT_RECORD: u32 = 3;
        const STACK_COOKIE: u32 = 4;
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        let count = rd32(this.wrapping_add(RECORD_COUNT));
        let service = rd32(this.wrapping_add(SERVICE_PTR));
        let mut decoded_record = 0u32;
        let mut decoded_aux = 0u32;
        let mut index = 0u32;
        while index < count {
            let record_ptr = rd32(
                this.wrapping_add(RECORDS)
                    .wrapping_add(index.wrapping_mul(4)),
            );
            let record_id = rd32(record_ptr);
            let accepted =
                lf_checker_rt::callee_thiscall!(ACCEPT_RECORD, u32, service, record_id) as u8 != 0;
            if accepted {
                let decoded = lf_checker_rt::callee_thiscall!(
                    DECODE_RECORD,
                    u32,
                    source,
                    (&mut decoded_record as *mut u32) as u32,
                    0,
                    (&mut decoded_aux as *mut u32) as u32,
                ) as u8
                    != 0;
                if decoded {
                    let handler = this.wrapping_add(HANDLER);
                    let _ = lf_checker_rt::callee_thiscall!(
                        COMMIT_RECORD,
                        u32,
                        handler,
                        record_id,
                        (&mut decoded_record as *mut u32) as u32,
                        decoded_aux,
                        0,
                        0,
                    );
                }
            }
            index = index.wrapping_add(1);
        }
        let cookie = lf_checker_rt::global::<u32>(STACK_COOKIE_VA).read();
        let _ = lf_checker_rt::callee_thiscall!(STACK_COOKIE, u32, cookie);
        0
    }
});
