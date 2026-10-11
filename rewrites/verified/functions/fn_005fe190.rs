// original: 0x005FE190 streaming_dispatch_state_query

/// Query the state helper and retain its full word result.
///
/// The receiver's context pointer is at `+0xb4`; the context's second word is
/// forwarded in EDX. The single stack argument points to a request whose
/// service pointer is at `+4`, and that pointer is forwarded in ECX. The
/// helper receives stack words `0xff` and zero; its EAX result is stored at
/// receiver offset `+0xc8` and returned unchanged. The helper uses ECX and EDX
/// with caller-cleaned stack arguments. The outer function uses thiscall.
lf_checker_rt::export!(thiscall, rw_005fe190(this: u32, request: u32) -> u32 {
    unsafe {
        const CONTEXT_POINTER: u32 = 0xb4;
        const REQUEST_SERVICE: u32 = 4;
        const CONTEXT_DATA: u32 = 8;
        const RESULT_WORD: u32 = 0xc8;
        const HELPER_ID: u32 = 1;

        let request_service =
            (request.wrapping_add(REQUEST_SERVICE) as *const u32).read_unaligned();
        let context = (this.wrapping_add(CONTEXT_POINTER) as *const u32).read_unaligned();
        let context_data = (context.wrapping_add(CONTEXT_DATA) as *const u32).read_unaligned();
        let result = lf_checker_rt::callee_fastcall!(
            HELPER_ID,
            u32,
            request_service,
            context_data,
            0xff,
            0,
        );
        (this.wrapping_add(RESULT_WORD) as *mut u32).write_unaligned(result);
        result
    }
});
