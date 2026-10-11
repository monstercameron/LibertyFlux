// original: 0x00D3DEC0 anim_request_helper_store_result (proposed)
/// Queries the request state for the object, writes a state code through
/// `result_out`, and clears the request fields for terminal states. The
/// thiscall method receives a pointer to one 32-bit output word on the stack;
/// the query is a direct cdecl call with the stored request handle and zero.
lf_checker_rt::export!(thiscall, rw_d3dec0(this: u32, result_out: u32) -> u8 {
    const QUERY_REQUEST: u32 = 1;
    const REQUEST_HANDLE_OFFSET: usize = 4;
    const STATE_OFFSET: usize = 0;

    let request_handle = unsafe { ((this as *const u8).add(REQUEST_HANDLE_OFFSET) as *const u32).read_unaligned() };
    let state = lf_checker_rt::callee_cdecl!(QUERY_REQUEST, u32, request_handle, 0);
    let result = result_out as *mut u32;
    unsafe {
        match state {
            0 => {
                result.write_unaligned(2);
                (this as *mut u8).add(STATE_OFFSET).write(0);
                ((this as *mut u8).add(REQUEST_HANDLE_OFFSET) as *mut u32).write_unaligned(0);
                1
            }
            1 => {
                result.write_unaligned(1);
                (this as *mut u8).add(STATE_OFFSET).write(0);
                ((this as *mut u8).add(REQUEST_HANDLE_OFFSET) as *mut u32).write_unaligned(0);
                1
            }
            2 => {
                result.write_unaligned(0);
                0
            }
            3 => {
                (this as *mut u8).add(STATE_OFFSET).write(0);
                ((this as *mut u8).add(REQUEST_HANDLE_OFFSET) as *mut u32).write_unaligned(0);
                result.write_unaligned(1);
                1
            }
            _ => 0,
        }
    }
});
