// original: 0x0091B740 text_encode_dispatched
/// Encode a string by one of two paths chosen from a global mode byte.
///
/// Measures the NUL-terminated input, then: when the mode byte differs from
/// `0x72` it forwards a scratch buffer, the two extra arguments, the string
/// and its length to the direct helper and returns its answer; when the mode
/// byte equals `0x72` it first maps the string through the byte helper into
/// the scratch buffer, calls the sizing helper with twice the third argument,
/// and finally calls the data-table target with `(0x4E3, 1, buffer, len, a1,
/// a2)`. On both paths the trailing stack-cookie check call runs last and its
/// answer is what `EAX` holds at return, so the rewrite returns that answer;
/// the cookie calls are reproduced as plain intercepted calls in the same
/// positions.
export!(cdecl, rw_0091b740(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let mut len: u32 = 0;
        while *((a0 + len) as *const u8) != 0 {
            len = len.wrapping_add(1);
        }
        let mut buf = [0u8; 516];
        let flag = *global::<u8>(0x116C250);
        if flag != 0x72 {
            let _: u32 = callee_cdecl!(1, u32, buf.as_mut_ptr() as u32, a1, a2, a0, len);
            callee_cdecl!(2, u32,)
        } else {
            let _: u32 = callee_cdecl!(3, u32, a0, buf.as_mut_ptr() as u32, 0x200);
            let _: u32 = callee_cdecl!(4, u32, a1, 0, a2.wrapping_mul(2));
            let tgt = *(relocated(0xE73270) as *const u32);
            let f: extern "stdcall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            let _: u32 = f(0x4E3, 1, buf.as_mut_ptr() as u32, len, a1, a2);
            callee_cdecl!(2, u32,)
        }
    }
});
