// original: 0x00A8AE90 pool_cookie_scan_dispatch

/// Open a named pool stream, resolve its handler object, and pass the buffer
/// plus the preceding stack word to the handler in that order. A null resolve
/// returns one. A handler mismatch returns its value with only the low byte
/// forced to one; a match forwards the word below the buffer to the pool
/// storage helper. Each exit runs the stack-cookie helper first.
///
/// `this` is the pool, `open_a` and `open_b` are forwarded to the open helper,
/// and `expected` is the handler result to match. The handler is called through
/// the object vtable at byte offset `0x74`. The proof zero-fills the preceding
/// local word and snapshots both pointer arguments. Cookie ECX depends on the
/// native frame, so that register is excluded from the cookie-call comparison.
lf_checker_rt::export!(thiscall, rw_00a8ae90(this: u32, open_a: u32, open_b: u32, expected: u32) -> u32 {
    unsafe {
        const CALLEE_OPEN: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_STORE: u32 = 3;
        const CALLEE_COOKIE: u32 = 5;
        const VTABLE_HANDLER_SLOT: u32 = 0x74;
        const BUFFER_BYTES: u32 = 0x200;

        let mut frame_words = [0u32; 129];
        let preceding_word = core::ptr::addr_of_mut!(frame_words[0]) as u32;
        let buffer = core::ptr::addr_of_mut!(frame_words[1]) as u32;

        lf_checker_rt::callee_cdecl!(
            CALLEE_OPEN,
            u32,
            buffer,
            BUFFER_BYTES,
            open_a,
            open_b,
            expected,
            0
        );
        let resolved = lf_checker_rt::callee_cdecl!(CALLEE_RESOLVE, u32, buffer, 1);
        if resolved == 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
            return 1;
        }

        let vtable = (resolved as *const u32).read_unaligned();
        let handler_address =
            ((vtable + VTABLE_HANDLER_SLOT) as *const u32).read_unaligned();
        let handler: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(handler_address as usize);
        let handler_result = handler(resolved, buffer, preceding_word);
        if handler_result != expected {
            lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
            return (handler_result & 0xffff_ff00) | 1;
        }

        let stored_word = (preceding_word as *const u32).read_unaligned();
        let result = lf_checker_rt::callee_thiscall!(CALLEE_STORE, u32, this, stored_word);
        lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
        result
    }
});
