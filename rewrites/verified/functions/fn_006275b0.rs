// original: 0x006275B0 network_record_parse_large_envelope

/// Parse a large envelope using two stack-local scratch buffers and an optional final transform.
/// The owner arrives in ECX and four words arrive on the stack; the second stack word is the first
/// parser's this pointer, while the first is forwarded to the final transform. The first parser
/// receives pointers to scratch words at offsets zero and four. If it succeeds, the owner parser
/// receives the first stack word, the second scratch pointer, the scratch value at offset zero,
/// and fixed mode values one and zero. Either parser may reject the envelope. Both exits verify
/// the relocated stack cookie, preserve the parser result in AL, and return after cleaning four
/// stack words.
lf_checker_rt::export!(thiscall, rw_006275b0(
    owner: u32,
    first_argument: u32,
    parser_owner: u32,
    _third_argument: u32,
    _fourth_argument: u32
) -> u32 {
    unsafe {
        const FIRST_PARSE: u32 = 1;
        const FINAL_PARSE: u32 = 2;
        const CHECK_STACK_COOKIE: u32 = 3;
        const STACK_COOKIE_VA: u32 = 0x0105_7FB4;

        let mut scratch = [0u32; 233];
        let scratch_base = (&mut scratch[0] as *mut u32 as usize) as u32;
        let scratch_plus_four = scratch_base.wrapping_add(4);

        let first_ok: u32 = lf_checker_rt::callee_thiscall!(
            FIRST_PARSE,
            u32,
            parser_owner,
            scratch_plus_four,
            parser_owner,
            scratch_base
        );
        let result = if first_ok & 0xFF == 0 {
            0
        } else {
            let second_ok: u32 = lf_checker_rt::callee_thiscall!(
                FINAL_PARSE,
                u32,
                owner,
                first_argument,
                scratch_plus_four,
                scratch[0],
                1,
                0
            );
            u32::from(second_ok & 0xFF != 0)
        };

        let cookie = (lf_checker_rt::global::<u32>(STACK_COOKIE_VA)).read_volatile();
        let _ = lf_checker_rt::callee_thiscall!(CHECK_STACK_COOKIE, u32, cookie);
        result
    }
});
