// original: 0x00E6D980 initialize_object_slots_00e6d980

/// Initialize a fixed array of 40 records at a 0x17c-byte stride. Before
/// the loop, write 0xffffffff to the global flag word at 0x017a6b20. For each
/// record, issue two scripted stdcall registrations: the first receives the
/// record address minus 0x38, 4, 7 and a callback address; the second receives
/// the record address, 8, 7 and a second callback address. Each callee pops its
/// four stack words. Return the last registration result. The two call sites are
/// reached 40 times each (80 logged calls total).

lf_checker_rt::export!(cdecl, rw_initialize_object_slots_00e6d980() -> u32 {
    unsafe {
        const FIRST_RECORD_VA: u32 = 0x017A6C64;
        const FIRST_FLAG_VA: u32 = 0x017A6B20;
        const RECORD_STRIDE: u32 = 0x17C;
        const RECORD_COUNT: u32 = 40;
        const FIRST_CALLBACK_VA: u32 = 0x004332D0;
        const SECOND_CALLBACK_VA: u32 = 0x004065E0;
        lf_checker_rt::global::<u32>(FIRST_FLAG_VA).write_unaligned(u32::MAX);
        let first_callback = lf_checker_rt::relocated(FIRST_CALLBACK_VA);
        let second_callback = lf_checker_rt::relocated(SECOND_CALLBACK_VA);
        let mut last_result = 0u32;
        for index in 0..RECORD_COUNT {
            let record = lf_checker_rt::relocated(FIRST_RECORD_VA)
                .wrapping_add(index.wrapping_mul(RECORD_STRIDE));
            let registration = record.wrapping_sub(0x38);
            let _first = lf_checker_rt::callee_stdcall!(
                1, u32, registration, 4u32, 7u32, first_callback
            );
            last_result = lf_checker_rt::callee_stdcall!(
                2, u32, record, 8u32, 7u32, second_callback
            );
        }
        last_result
    }
});
