// original: 0X0066FE70 rage::fiTokenizer::vf25
/// Fetch one token, compare it against the expected string, then forward to virtual slot `+0x28` with (`value`, 1) and return that answer. `this` points to the tokenizer, `expected` to the expected string and `value` is forwarded. The fetch goes to slot `+0x08` with a zero-initialised 0x40-byte frame buffer and its length; when it reports a token, the string routine is called with (`expected`, token) and its result is ignored.
///
/// Original: 0X0066FE70 (thiscall, 2 stack arguments, 3 calls).
lf_checker_rt::export!(thiscall, rw_0066fe70(this: u32, expected: u32, value: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x40;
        const FORWARD_SLOT: u32 = 0x28;
        const STRCMP_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let get_token: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + TOKEN_SLOT) as usize);
        let forward: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + FORWARD_SLOT) as usize);
        let mut buf = [0u32; 16];
        let len = get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN);
        if len != 0 {
            lf_checker_rt::callee_cdecl!(STRCMP_CALLEE, u32, expected, buf.as_mut_ptr() as u32);
        }
        forward(this, value, 1)
    }
});
