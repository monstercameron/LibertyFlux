// original: 0X0066FC50 rage::fiTokenizer::vf13
/// Fetch one token, discard it, then forward to virtual slot `+0x14` with flag 1 and return that answer. `this` points to the tokenizer; the single stack argument is ignored. The fetch goes to slot `+0x08` with a zero-initialised 0x40-byte frame buffer and its length; the buffered bytes are not read.
///
/// Original: 0X0066FC50 (thiscall, 1 stack argument, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066fc50(this: u32, _ignored: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x40;
        const FORWARD_SLOT: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let vtable = rd32(this);
        let get_token: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + TOKEN_SLOT) as usize);
        let forward: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(rd32(vtable + FORWARD_SLOT) as usize);
        let mut buf = [0u32; 16];
        get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN);
        forward(this, 1)
    }
});
