// original: 0x00E6E150 mainloop_timing_release_resource_group_b

/// Visit two resource records in reverse fixed-stride order. For each record,
/// close its handle when the handle word is nonzero, then release the record's
/// critical section. Both calls use the stdcall convention; the result is the
/// EAX value from the final critical-section cleanup call.
lf_checker_rt::export!(cdecl, rw_00e6e150() -> u32 {
    const DELETE_CRITICAL_SECTION_ID: u32 = 1;
    const CLOSE_HANDLE_ID: u32 = 2;
    const RESOURCE_ORIGIN_VA: u32 = 0x018c_bfc8;
    const HANDLE_FIELD_OFFSET: u32 = 0x6144;
    const RESOURCE_STRIDE_BYTES: u32 = 0x6170;

    let mut resource_va = RESOURCE_ORIGIN_VA;
    let mut last_delete_result = 0;
    for _ in 0..2 {
        let handle_va = resource_va.wrapping_sub(HANDLE_FIELD_OFFSET);
        let handle = unsafe { lf_checker_rt::global::<u32>(handle_va).read() };
        resource_va = resource_va.wrapping_sub(RESOURCE_STRIDE_BYTES);
        if handle != 0 {
            let _ = lf_checker_rt::callee_stdcall!(CLOSE_HANDLE_ID, u32, handle);
        }
        last_delete_result = lf_checker_rt::callee_stdcall!(
            DELETE_CRITICAL_SECTION_ID,
            u32,
            lf_checker_rt::relocated(resource_va)
        );
    }
    last_delete_result
});
