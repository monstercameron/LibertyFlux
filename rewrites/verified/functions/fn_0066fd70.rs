// original: 0X0066FD70 rage::fiTokenizer::vf15
/// Fetch one token, discard it, then forward to virtual slot `+0x1C` with (`value`, 1) and return that answer. `this` points to the tokenizer; the first stack argument is ignored and the second is forwarded. The fetch goes to slot `+0x08` with a zero-initialised 0x40-byte frame buffer and its length; the buffered bytes are not read.
///
/// Original: 0X0066FD70 (thiscall, 2 stack arguments, 2 calls).
lf_checker_rt::export!(thiscall, rw_0066fd70(this: u32, _name: u32, value: u32) -> u32 {
    unsafe {
        const TOKEN_SLOT: u32 = 0x08;
        const TOKEN_LEN: u32 = 0x40;
        const FORWARD_SLOT: u32 = 0x1c;

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
        get_token(this, buf.as_mut_ptr() as u32, TOKEN_LEN);
        forward(this, value, 1)
    }
});
