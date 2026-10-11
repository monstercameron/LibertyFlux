// original: 0x006271A0 network_record_walk_entries_and_set_fields

/// Validate a record header, request its bounded entry count, check each eight-byte entry,
/// then run two mode-specific field reads. The owner and record arrive in ECX and EDX. The
/// count is an unsigned word written through the request helper at record +0x10C; counts above
/// 32 return false before changing record +0x108. The entry helper receives record +0x114 plus
/// eight bytes per index and stops the walk at its first false result. The two final helpers
/// receive a stack-local two-word output buffer and modes 1 and 15. The first output word is
/// written to record +0x108 after both reads have been attempted. The return is a low-byte
/// boolean requiring every entry and both field reads to succeed. The original also uses a
/// zero-filled uninitialized local word in the output calculation; the proof fixes that stack
/// fill to zero and this rewrite expresses the resulting calculation explicitly.
lf_checker_rt::export!(fastcall, rw_006271a0(owner: u32, record: u32) -> u32 {
    unsafe {
        const COUNT_FIELD: u32 = 0x10C;
        const OUTPUT_FIELD: u32 = 0x108;
        const FIRST_ENTRY: u32 = 0x114;
        const ENTRY_STRIDE: u32 = 8;
        const MAX_ENTRIES: u32 = 32;
        const REQUEST_SIZE: u32 = 6;
        const FIRST_MODE: u32 = 1;
        const SECOND_MODE: u32 = 15;
        const HEADER_CHECK: u32 = 1;
        const REQUEST_COUNT: u32 = 2;
        const CHECK_ENTRY: u32 = 3;
        const READ_FIRST_FIELD: u32 = 4;
        const READ_SECOND_FIELD: u32 = 5;
        const STACK_FILL_WORD: u32 = 0;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let header_ok: u32 = lf_checker_rt::callee_fastcall!(HEADER_CHECK, u32, owner, record);
        if header_ok & 0xFF == 0 {
            return 0;
        }

        let count_address = record.wrapping_add(COUNT_FIELD);
        let _ = lf_checker_rt::callee_thiscall!(
            REQUEST_COUNT,
            u32,
            owner,
            count_address,
            REQUEST_SIZE
        );
        let count = read_u32(count_address);
        if count > MAX_ENTRIES {
            return 0;
        }

        let mut entries_valid = true;
        let mut index = 0u32;
        while index < count {
            let entry = record
                .wrapping_add(FIRST_ENTRY)
                .wrapping_add(index.wrapping_mul(ENTRY_STRIDE));
            let entry_ok: u32 =
                lf_checker_rt::callee_fastcall!(CHECK_ENTRY, u32, owner, entry);
            if entry_ok & 0xFF == 0 {
                entries_valid = false;
                break;
            }
            index = index.wrapping_add(1);
        }

        let mut field_words = [0u32; 2];
        let first_field_ok: u32 = lf_checker_rt::callee_thiscall!(
            READ_FIRST_FIELD,
            u32,
            owner,
            field_words.as_mut_ptr() as usize as u32,
            FIRST_MODE
        );
        let fields_valid = if first_field_ok & 0xFF != 0 {
            field_words = [0; 2];
            let second_field_ok: u32 = lf_checker_rt::callee_thiscall!(
                READ_SECOND_FIELD,
                u32,
                owner,
                field_words.as_mut_ptr() as usize as u32,
                SECOND_MODE
            );
            second_field_ok & 0xFF != 0
        } else {
            false
        };

        let stack_word = STACK_FILL_WORD;
        let output_value = stack_word.wrapping_add(stack_word.wrapping_neg() ^ field_words[0]);
        write_u32(record.wrapping_add(OUTPUT_FIELD), output_value);

        if entries_valid && fields_valid {
            1
        } else {
            0
        }
    }
});
