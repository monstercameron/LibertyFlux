// original: 0X0066FC30 rage::fiTokenizer::vf12
/// Fetch one token into a frame buffer and return the fetch result. `this` points to the tokenizer; the single stack argument is ignored. Virtual slot `+0x08` is called with a zero-initialised 0x40-byte frame buffer and its length, and its answer is returned. The buffered bytes are not read.
///
/// Original: 0X0066FC30 (thiscall, 1 stack argument, 1 call).
lf_checker_rt::export!(thiscall, rw_0066fc30(this: u32, _ignored: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x40;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let get_token: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + TOKEN_SLOT) as usize);
        let mut buf = [0u32; 16];
        get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN)
    }
});
