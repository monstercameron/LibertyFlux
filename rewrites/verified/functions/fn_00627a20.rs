// original: 0x00627A20 network_record_parse_small_envelope

/// Clear a 64-byte stack buffer, parse one record into it with a stack-local count, then pass the
/// parsed byte count (count times eight) and buffer to the record processor. The owner and input
/// record arrive in ECX and EDX. Parsing failure returns false; processing runs only after a
/// successful parse, and its AL result is returned. A relocated stack cookie is checked on both
/// exits. Pointer arguments to stack buffers are skipped in the call log; their 64-byte contents
/// and the count word are captured at each call. The stack-content check is disabled for the local
/// scratch buffer and cookie slot.
lf_checker_rt::export!(fastcall, rw_00627a20(owner: u32, record: u32) -> u32 {
    unsafe {
        const MEMSET: u32 = 1;
        const PARSE_RECORD: u32 = 2;
        const PROCESS_RECORD: u32 = 3;
        const CHECK_STACK_COOKIE: u32 = 4;
        const STACK_COOKIE_VA: u32 = 0x0105_7FB4;
        const BUFFER_BYTES: u32 = 0x40;
        const CLEARED_BYTES: u32 = 0x3F;

        let mut backing = [0u8; 65];
        let parser_buffer = backing.as_mut_ptr() as usize as u32;
        let memset_buffer = parser_buffer.wrapping_add(1);
        let _: u32 = lf_checker_rt::callee_cdecl!(MEMSET, u32, memset_buffer, 0, CLEARED_BYTES);

        let mut parsed_count = 0u32;
        let parse_ok: u32 = lf_checker_rt::callee_thiscall!(
            PARSE_RECORD,
            u32,
            record,
            parser_buffer,
            BUFFER_BYTES,
            &mut parsed_count as *mut u32 as usize as u32
        );
        let result = if parse_ok & 0xFF == 0 {
            0
        } else {
            let byte_count = parsed_count.wrapping_shl(3);
            let processed: u32 = lf_checker_rt::callee_thiscall!(
                PROCESS_RECORD,
                u32,
                owner,
                parser_buffer,
                byte_count,
                record
            );
            processed & 0xFF
        };

        let cookie = (lf_checker_rt::global::<u32>(STACK_COOKIE_VA)).read_volatile();
        let _: u32 = lf_checker_rt::callee_thiscall!(CHECK_STACK_COOKIE, u32, cookie);
        result
    }
});
