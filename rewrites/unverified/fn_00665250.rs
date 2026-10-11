// original: 0x00665250 network_dispatch_if_valid

/// Parse one network message and pass its decoded fields to the owning object.
///
/// `this` is the destination object. `source` is the original message source,
/// `parser_context` supplies the decoding service, the third stack argument is
/// unused, and `options` is forwarded to the destination handler. The decoder
/// receives two temporary 32-bit output slots. A rejected message returns
/// false without dispatch; an accepted parse is dispatched with the fixed
/// 2,000-byte bound and returns the handler's low-byte success value. The
/// method uses the 32-bit thiscall ABI and pops four stack arguments.
lf_checker_rt::export!(thiscall, rw_00665250(this: u32, source: u32, parser_context: u32, _unused: u32, options: u32) -> u32 {
    unsafe {
        const PARSE_MESSAGE: u32 = 1;
        const DELIVER_MESSAGE: u32 = 2;
        const STACK_COOKIE: u32 = 3;
        const STACK_COOKIE_VA: u32 = 0x0105_7fb4;
        const MAX_MESSAGE_BYTES: u32 = 0x7d0;

        let mut parsed_length = 0u32;
        let mut parsed_flags = 0u32;
        let parsed = lf_checker_rt::callee_thiscall!(
            PARSE_MESSAGE,
            u32,
            parser_context,
            (&mut parsed_flags as *mut u32) as u32,
            parser_context,
            (&mut parsed_length as *mut u32) as u32,
        ) as u8 != 0;
        let accepted = if parsed {
            lf_checker_rt::callee_thiscall!(
                DELIVER_MESSAGE,
                u32,
                this,
                source,
                (&mut parsed_flags as *mut u32) as u32,
                parsed_length,
                MAX_MESSAGE_BYTES,
                options,
            ) as u8 != 0
        } else {
            false
        };
        let cookie = lf_checker_rt::global::<u32>(STACK_COOKIE_VA).read();
        let _ = lf_checker_rt::callee_thiscall!(STACK_COOKIE, u32, cookie);
        u32::from(accepted)
    }
});
