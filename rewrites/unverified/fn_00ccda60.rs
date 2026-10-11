// original: 0x00CCDA60 task_method_00ccda60

/// Passing stage for the elapsed-time check and one visibility query. The
/// current and reference times are relocated image globals; elapsed time is
/// an unsigned wrapping difference. The contract covers values on both sides
/// of 2,000, clears the task's query-suppression flag, and arranges for the
/// query helper to return false. Its local output buffer is zeroed and
/// snapshotted at the call. Other query results and later state changes remain
/// unimplemented.
///
/// ABI: thiscall with the task in ECX and the event pointer on the stack.
pub(crate) unsafe fn run(task: u32, event: u32) -> u32 {
    const CURRENT_TIME_VA: u32 = 0x0117_35b4;
    const REFERENCE_TIME_VA: u32 = 0x0117_35b8;
    const TASK_TIME: u32 = 0x14;
    const TASK_FLAGS: u32 = 0x18;
    const SUPPRESS_QUERY: u8 = 0x04;
    const QUERY_DELAY_MS: u32 = 2000;
    const EVENT_VECTOR_PTR: u32 = 0x20;
    const EVENT_EXIT_CODE: u32 = 0x7a8;
    const VECTOR_X: u32 = 0x30;
    const VECTOR_Y: u32 = 0x34;
    const VECTOR_Z: u32 = 0x38;
    const QUERY: u32 = 1;

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    unsafe {
        let now = read_u32(lf_checker_rt::relocated(CURRENT_TIME_VA));
        let elapsed = now.wrapping_sub(read_u32(task.wrapping_add(TASK_TIME)));
        if elapsed <= QUERY_DELAY_MS {
            return elapsed;
        }
        if read_u8(task.wrapping_add(TASK_FLAGS)) & SUPPRESS_QUERY != 0 {
            return elapsed;
        }

        let age_after_delay = elapsed.wrapping_sub(QUERY_DELAY_MS);
        let reference_span = now.wrapping_sub(read_u32(lf_checker_rt::relocated(REFERENCE_TIME_VA)));
        if age_after_delay > reference_span {
            return elapsed;
        }

        let vector = read_u32(event.wrapping_add(EVENT_VECTOR_PTR));
        let output_words = [0u32; 4];
        let output_pointer = output_words.as_ptr() as u32;
        let query_result = lf_checker_rt::callee_cdecl!(
            QUERY,
            u32,
            read_u32(vector.wrapping_add(VECTOR_X)),
            read_u32(vector.wrapping_add(VECTOR_Y)),
            read_u32(vector.wrapping_add(VECTOR_Z)),
            output_pointer,
            0x40c0_0000u32,
            0x41a0_0000u32,
            0u32
        );
        if query_result & 0xff != 0 {
            return query_result;
        }
        if read_u8(task.wrapping_add(TASK_FLAGS)) & SUPPRESS_QUERY != 0 {
            return query_result;
        }
        if (read_u32(event.wrapping_add(EVENT_EXIT_CODE)) as i32) <= -1 {
            return query_result;
        }
        query_result
    }
}

lf_checker_rt::export!(thiscall, rw_00ccda60(task: u32, event: u32) -> u32 {
    unsafe { run(task, event) }
});
