// original: 0x006272A0 network_record_write_field_validity

/// Check the record header, read one field through a stack-local output word, and store a
/// one-byte validity flag at record +8. The owner and record arrive in ECX and EDX. A false
/// header result returns immediately without changing the record. The field helper receives
/// mode 1; its AL result controls the function return, while its output word independently
/// controls the stored flag. The stack-local output pointer is skipped in the call log and its
/// initial word is snapshotted. The proof disables stack-content comparison because the
/// original uses that local word over a saved register slot; the record byte and call snapshot
/// observe the relevant value.
lf_checker_rt::export!(fastcall, rw_006272a0(owner: u32, record: u32) -> u32 {
    unsafe {
        const RECORD_FLAG: u32 = 8;
        const FIELD_MODE: u32 = 1;
        const CHECK_HEADER: u32 = 1;
        const READ_FIELD: u32 = 2;

        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write_unaligned(value) }
        }

        let header_ok: u32 = lf_checker_rt::callee_fastcall!(CHECK_HEADER, u32, owner, record);
        if header_ok & 0xFF == 0 {
            return 0;
        }

        let mut field_value = 0u32;
        let field_read_ok: u32 = lf_checker_rt::callee_thiscall!(
            READ_FIELD,
            u32,
            owner,
            &mut field_value as *mut u32 as usize as u32,
            FIELD_MODE
        );
        let field_valid = field_read_ok & 0xFF != 0 && field_value != 0;
        write_u8(record.wrapping_add(RECORD_FLAG), field_valid as u8);

        if field_read_ok & 0xFF != 0 {
            1
        } else {
            0
        }
    }
});
