// original: 0x00D3E5A0 task_request_create_from_arguments (proposed)
/// Prepares the request object, forwards six input words plus the fixed mode
/// value one to a seven-argument cdecl task factory, and stores the returned
/// pointer at offset 4. The leading byte becomes one only for a non-null
/// factory result; that byte is also the function's return value. The
/// preparation and factory calls are both checked through stand-ins.
lf_checker_rt::export!(thiscall, rw_d3e5a0(
    result_out: u32,
    argument_0: u32,
    argument_1: u32,
    argument_2: u32,
    argument_3: u32,
    argument_4: u32,
    argument_5: u32,
) -> u8 {
    const PREPARE_REQUEST: u32 = 1;
    const CREATE_TASK: u32 = 2;
    const TASK_POINTER_OFFSET: usize = 4;
    let _ = lf_checker_rt::callee_thiscall!(PREPARE_REQUEST, u32, result_out);
    let task = lf_checker_rt::callee_cdecl!(
        CREATE_TASK,
        u32,
        argument_0,
        argument_1,
        1,
        argument_2,
        argument_3,
        argument_4,
        argument_5,
    );
    unsafe {
        ((result_out + TASK_POINTER_OFFSET as u32) as *mut u32).write_unaligned(task);
        if task != 0 {
            (result_out as *mut u8).write(1);
        }
        (result_out as *const u8).read()
    }
});
