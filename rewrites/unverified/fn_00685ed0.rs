// original: 0x00685ED0 track_apply_request (proposed)

/// Forward a track request to the animation dispatcher. The outer thiscall
/// receives a pointer to a two-word request at `+0` and a second 32-bit
/// value on the stack. It reads the request's word at `+4`, then calls the
/// dispatcher with a temporary two-word context `[outer_this, request]` in
/// ECX and three stack arguments `[request_word, second_value, outer_this]`.
/// The temporary context is observed by a call-time snapshot because its
/// stack address differs between the original and the Rust frame. The
/// dispatcher's 32-bit result is returned unchanged in EAX.
lf_checker_rt::export!(thiscall, rw_00685ED0(outer_this: u32, request: u32, second_value: u32) -> u32 {
    unsafe {
        const DISPATCHER_ID: u32 = 1;
        const REQUEST_WORD: u32 = 0x04;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let request_word = unsafe { read_u32(request + REQUEST_WORD) };
        let mut dispatcher_context = [outer_this, request];
        unsafe {
            lf_checker_rt::callee_thiscall!(
                DISPATCHER_ID,
                u32,
                dispatcher_context.as_mut_ptr() as usize as u32,
                request_word,
                second_value,
                outer_this,
            )
        }
    }
});
