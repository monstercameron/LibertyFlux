// original: 0x009B3EB0 NativeImpl_REPORT_CRIME

/// Reports a crime when the scanner's global reporting flag is enabled.
/// The second argument is the crime category and the third is its identifier.
/// It resolves that identifier through the static report manager, stops if no
/// record exists, then builds a report payload and sends it through the
/// scanner's formatting and report helpers. The payload stores category
/// metadata at byte offset 20, the resolved record at 24, a delay at 28, and
/// two zero words at 32 and 36; it sets the low byte at offset 40. The helper
/// calls are stand-ins in the proof contract; the wrapper has a thiscall
/// interface and pops two words.
lf_checker_rt::export!(thiscall, rw_009b3eb0(this: u32, crime_type: u32, crime_id: u32) -> u32 {
    const REPORTING_ENABLED: u32 = 0x01039238;
    const REPORT_MANAGER: u32 = 0x0115d9a0;
    const PAYLOAD_SIZE: usize = 0x38;
    const LOOKUP_RECORD: usize = 0x18;
    const CATEGORY_METADATA: usize = 0x14;
    const REPORT_DELAY: usize = 0x1c;
    const REPORT_FLAGS: usize = 0x20;
    const REPORT_STATE: usize = 0x24;
    const REPORT_ACTIVE: usize = 0x28;
    const CATEGORY_METADATA_VALUE: u32 = 4;
    const REPORT_DELAY_MS: u32 = 1000;

    unsafe {
        if lf_checker_rt::global::<u8>(REPORTING_ENABLED).read() == 0 {
            return 0;
        }

        let record = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            lf_checker_rt::relocated(REPORT_MANAGER),
            crime_id
        );
        if record == 0 {
            return 0;
        }

        let mut payload = [0u8; PAYLOAD_SIZE];
        let payload_address = payload.as_mut_ptr() as usize as u32;
        (payload_address.wrapping_add(LOOKUP_RECORD as u32) as *mut u32)
            .write_unaligned(record);
        (payload_address.wrapping_add(CATEGORY_METADATA as u32) as *mut u32)
            .write_unaligned(CATEGORY_METADATA_VALUE);
        (payload_address.wrapping_add(REPORT_DELAY as u32) as *mut u32)
            .write_unaligned(REPORT_DELAY_MS);
        (payload_address.wrapping_add(REPORT_FLAGS as u32) as *mut u32)
            .write_unaligned(0);
        (payload_address.wrapping_add(REPORT_STATE as u32) as *mut u32)
            .write_unaligned(0);
        (payload_address.wrapping_add(REPORT_ACTIVE as u32) as *mut u8).write(1);

        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, crime_type, payload_address);
        let _ = lf_checker_rt::callee_thiscall!(3, u32, this, payload_address);
        0
    }
});
