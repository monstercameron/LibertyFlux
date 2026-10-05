// original: 0x00a8ae90 pool_load_named (proposed)

/// Load the named pool stream, verifying it through its handler.
///
/// `this` is the pool; `a1`, `a2` and `a3` are the open arguments. Opens
/// the stream through the open callee into a frame buffer (its address is
/// uncompared; its first word feeds the store callee's argument), then
/// resolves the buffer through the resolve callee: a null answer fails.
/// Otherwise the handler at the answer's slot runs on the two buffer
/// words, and its answer must equal `a3`; then the answer word is stored
/// through the store callee. Failure returns 1, success the store
/// answer. Both exits run the cookie check first (modelled as a
/// register-preserving call). The proof cycles null and live resolve
/// answers and matching and mismatching handler answers.
///
/// Original: 0x00A8AE90 (thiscall, three stack words, register-indirect
/// handler, stack cookie).
lf_checker_rt::export!(thiscall, rw_00a8ae90(this: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const CALLEE_OPEN: u32 = 1;
        const CALLEE_RESOLVE: u32 = 2;
        const CALLEE_STORE: u32 = 3;
        const CALLEE_HANDLER: u32 = 4;
        const CALLEE_COOKIE: u32 = 5;
        const HANDLER_SLOT: u32 = 0x74;
        const BUF_SIZE: u32 = 0x200;
        let mut buf = [0u32; 2];
        let buf_addr = core::ptr::addr_of_mut!(buf) as u32;
        lf_checker_rt::callee_cdecl!(
            CALLEE_OPEN,
            u32,
            buf_addr,
            BUF_SIZE,
            a2,
            a3,
            a1,
            0
        );
        let buf1 = core::ptr::addr_of_mut!(buf[1]) as u32;
        let resolved =
            lf_checker_rt::callee_cdecl!(CALLEE_RESOLVE, u32, buf1, 1);
        if resolved == 0 {
            lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
            return 1;
        }
        let vtable = (resolved as *const u32).read_unaligned();
        let target = ((vtable + HANDLER_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let checked = f(resolved, buf1, buf_addr);
        if checked != a3 {
            lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
            return 1;
        }
        let r = lf_checker_rt::callee_thiscall!(
            CALLEE_STORE,
            u32,
            this,
            buf[0]
        );
        lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
        r
    }
});
