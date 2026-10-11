// original: 0x00665390 network_route_if_accepted

/// Read message metadata through the parser context and route accepted data to
/// this object. The parser receives the context in ECX and fills two local
/// 32-bit output slots. On parser failure the method returns false. Otherwise
/// it forwards the source, one output slot by pointer, and the other by value,
/// returning the handler's low-byte result. This is 32-bit thiscall with two
/// stack arguments.
lf_checker_rt::export!(thiscall, rw_00665390(this: u32, source: u32, parser_context: u32) -> u32 {
    unsafe {
        const READ_METADATA: u32 = 1;
        const ROUTE_MESSAGE: u32 = 2;
        const STACK_COOKIE: u32 = 3;
        const STACK_COOKIE_VA: u32 = 0x0105_7fb4;
        let mut parsed_length = 0u32;
        let mut parsed_flags = 0u32;
        let parsed = lf_checker_rt::callee_thiscall!(
            READ_METADATA,
            u32,
            parser_context,
            (&mut parsed_flags as *mut u32) as u32,
            parser_context,
            (&mut parsed_length as *mut u32) as u32,
        ) as u8
            != 0;
        let accepted = if parsed {
            lf_checker_rt::callee_thiscall!(
                ROUTE_MESSAGE,
                u32,
                this,
                source,
                (&mut parsed_flags as *mut u32) as u32,
                parsed_length,
            ) as u8
                != 0
        } else {
            false
        };
        let cookie = lf_checker_rt::global::<u32>(STACK_COOKIE_VA).read();
        let _ = lf_checker_rt::callee_thiscall!(STACK_COOKIE, u32, cookie);
        u32::from(accepted)
    }
});
