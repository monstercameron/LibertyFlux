// original: 0x0086D070 Script_ThreadFail
/// Record a thread failure code, report the failure through the optional
/// global callback, set the thread state to the failed state, and return 2.
/// The callback is a cdecl function pointer stored in a global word and, when
/// present, receives `(failure_code, thread_object)`.
lf_checker_rt::export!(thiscall, rw_0086D070(context: u32, failure_code: u32) -> u32 {
    const CALLBACK_POINTER_VA: u32 = 0x018B_9B44;
    const THREAD_STATE_OFFSET_WORDS: usize = 0x0C / 4;
    const THREAD_FAILURE_OFFSET_WORDS: usize = 0x6C / 4;
    const FAILURE_STATE: u32 = 2;
    unsafe {
        let object = context as *mut u32;
        object.add(THREAD_FAILURE_OFFSET_WORDS).write_unaligned(failure_code);
        let callback_pointer = lf_checker_rt::global::<u32>(CALLBACK_POINTER_VA)
            .read_unaligned();
        if callback_pointer != 0 {
            let report_failure: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(callback_pointer as usize);
            let _ = report_failure(failure_code, context);
        }
        object.add(THREAD_STATE_OFFSET_WORDS).write_unaligned(FAILURE_STATE);
    }

    FAILURE_STATE
});
