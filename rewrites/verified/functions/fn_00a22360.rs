// original: 0x00a22360 aim_record_copy_payload_and_set_enabled

/// Set the aim record's active bit and copy its four-word payload from the
/// source record. The final copied word is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00a22360(this: u32, source: u32) -> u32 {
    const FLAGS: u32 = 0x38d;
    const PAYLOAD: u32 = 0x270;
    const ACTIVE: u8 = 0x10;
    unsafe {
        let flags = (this.wrapping_add(FLAGS) as *const u8).read();
        (this.wrapping_add(FLAGS) as *mut u8).write(flags | ACTIVE);
        let mut word = 0u32;
        while word < 4 {
            let value = (source.wrapping_add(word * 4) as *const u32).read_unaligned();
            (this.wrapping_add(PAYLOAD + word * 4) as *mut u32).write_unaligned(value);
            word += 1;
        }
        (source.wrapping_add(12) as *const u32).read_unaligned()
    }
});
