// original: 0x006652D0 network_process_pending_records

/// Visit the object’s pending record pointers and pass nonnegative record
/// identifiers through a sequence of object callbacks. The signed count at
/// `this + 0x2ea4` bounds the pointer table at `this + 0x2e24`; the callback
/// service pointer is stored at `this + 0x24`. Negative record identifiers
/// are skipped. Each nonnegative identifier is accepted by either of the two
/// one-argument callbacks, then decoded into two temporary words. A successful
/// decode forwards one decoded word by pointer and the other by value, with
/// fixed flags `1, 0`.
/// The method is 32-bit thiscall with two stack arguments and a void result.
lf_checker_rt::export!(thiscall, rw_006652D0(this: u32, source: u32, _unused: u32) -> u32 {
    unsafe {
        const SERVICE_PTR: u32 = 0x24;
        const RECORDS: u32 = 0x2e24;
        const RECORD_COUNT: u32 = 0x2ea4;
        const STACK_COOKIE_VA: u32 = 0x0105_7fb4;
        const ACCEPT_RECORD: u32 = 1;
        const ACCEPT_FALLBACK: u32 = 2;
        const DECODE_RECORD: u32 = 3;
        const COMMIT_RECORD: u32 = 4;
        const STACK_COOKIE: u32 = 5;

        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let count = rd32(this.wrapping_add(RECORD_COUNT)) as i32;
        let service = rd32(this.wrapping_add(SERVICE_PTR));
        let mut decoded_record = 0u32;
        let mut decoded_aux = 0u32;
        let mut index = 0i32;
        while index < count {
            let record_ptr = rd32(
                this.wrapping_add(RECORDS)
                    .wrapping_add((index as u32).wrapping_mul(4)),
            );
            let record_id = rd32(record_ptr);
            if (record_id as i32) >= 0 {
                let accepted =
                    lf_checker_rt::callee_thiscall!(ACCEPT_RECORD, u32, service, record_id) as u8
                        != 0
                        || lf_checker_rt::callee_thiscall!(
                            ACCEPT_FALLBACK,
                            u32,
                            service,
                            record_id
                        ) as u8
                            != 0;
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
                        let _ = lf_checker_rt::callee_thiscall!(
                            COMMIT_RECORD,
                            u32,
                            service,
                            record_id,
                            (&mut decoded_record as *mut u32) as u32,
                            decoded_aux,
                            1,
                            0,
                        );
                    }
                }
            }
            index = index.wrapping_add(1);
        }
        let cookie = lf_checker_rt::global::<u32>(STACK_COOKIE_VA).read();
        let _ = lf_checker_rt::callee_thiscall!(STACK_COOKIE, u32, cookie);
        0
    }
});
