// original: 0x00B8F8B0 NativeImpl_GET_FRONTEND_DESIGN_VALUE

/// Read the first returned word and the second returned word from two service records, storing one word through each output pointer.
lf_checker_rt::export!(cdecl, rw_00b8f8b0(selector: u32, out_first: u32, out_second: u32) -> () {
    let mut scratch_words = [0u32; 2];
    let first_record = lf_checker_rt::callee_cdecl!(1, u32, scratch_words.as_mut_ptr() as u32, selector);
    let first_value = unsafe { core::ptr::read_unaligned(first_record as *const u32) };
    unsafe { core::ptr::write_unaligned(out_first as *mut u32, first_value); }
    let second_record = lf_checker_rt::callee_cdecl!(2, u32, scratch_words.as_mut_ptr() as u32);
    let second_value = unsafe { core::ptr::read_unaligned((second_record as *const u32).add(1)) };
    unsafe { core::ptr::write_unaligned(out_second as *mut u32, second_value); }
});
