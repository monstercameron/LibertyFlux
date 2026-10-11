// original: 0x00D44890 ped_task_resolve_profile_values

/// Resolve the first matching pedestrian task profile into three output words.
///
/// `ped` is a CPed pointer; the flags word at `ped + 0x24` controls a special
/// profile override. The function asks its profile predicate for candidates
/// 0 through 26, stopping at the first match. A miss copies three default
/// words; a match copies a three-word table entry. Candidate 1 replaces that
/// entry with the fixed special tuple when elapsed time is greater than or
/// equal to its limit and the pedestrian flag is clear. The float comparison
/// is ordered, so NaN does not qualify. Output words are copied bit-for-bit.
/// The cdecl arguments are the ped pointer followed by three writable word
/// pointers; the function has no meaningful return value.
lf_checker_rt::export!(cdecl, rw_ped_task_resolve_profile_values(
    ped: u32,
    out_first: u32,
    out_second: u32,
    out_third: u32
) -> () {
    unsafe {
        const PROFILE_LIMIT: u32 = 27;
        const PROFILE_RECORDS: usize = 50;
        const RECORD_WORDS: usize = 9;
        const PED_FLAGS: u32 = 0x24;
        const SPECIAL_FLAG: u32 = 0x0800_0000;
        const PROFILE_TABLE: u32 = 0x0172_07A8;
        const DEFAULT_FIRST: u32 = 0x0105_4CD4;
        const ELAPSED_TIME: u32 = 0x012D_DEAC;
        const SPECIAL_LIMIT: u32 = 0x00FE_87D0;
        const SECURITY_COOKIE: u32 = 0x0105_7FB4;
        const SPECIAL_FIRST: u32 = 0x4020_0000;
        const SPECIAL_SECOND: u32 = 0;
        const SPECIAL_THIRD: u32 = 0x42A0_0000;

        #[inline(always)]
        unsafe fn read_global_word(address: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(address).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut candidate_records = [0u32; PROFILE_RECORDS * RECORD_WORDS];
        let mut candidate_output = [0u32; 1];
        let record_base = candidate_records.as_mut_ptr() as u32;
        let mut selected = None;

        for index in 0..PROFILE_LIMIT {
            let matched: u8 = lf_checker_rt::callee_cdecl!(
                1,
                u8,
                ped,
                1u32,
                index,
                candidate_output.as_mut_ptr() as u32,
                0u32,
                0u32,
                record_base,
            );
            if matched != 0 {
                selected = Some(index);
                break;
            }
        }

        match selected {
            None => {
                write_word(out_first, read_global_word(DEFAULT_FIRST));
                write_word(out_second, read_global_word(DEFAULT_FIRST + 4));
                write_word(out_third, read_global_word(DEFAULT_FIRST + 8));
            }
            Some(index) => {
                let entry = PROFILE_TABLE + index * 12;
                write_word(out_first, read_global_word(entry));
                write_word(out_second, read_global_word(entry + 4));
                write_word(out_third, read_global_word(entry + 8));

                if index == 1 {
                    let elapsed = f32::from_bits(read_global_word(ELAPSED_TIME));
                    let limit = f32::from_bits(read_global_word(SPECIAL_LIMIT));
                    let flags = read_word(ped + PED_FLAGS);
                    if elapsed >= limit && flags & SPECIAL_FLAG == 0 {
                        write_word(out_first, SPECIAL_FIRST);
                        write_word(out_second, SPECIAL_SECOND);
                        write_word(out_third, SPECIAL_THIRD);
                    }
                }
            }
        }

        let cookie = read_global_word(SECURITY_COOKIE);
        let _: () = lf_checker_rt::callee_thiscall!(2, (), cookie);
    }
});
