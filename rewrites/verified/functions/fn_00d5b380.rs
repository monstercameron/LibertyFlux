// original: 0x00D5B380 loader_tls_resource_scope (proposed)

/// Temporarily increments the per-thread resource object's reference field,
/// calls the resource initializer with the supplied object's field at offset
/// `0x1c`, then decrements the reference field and returns the TLS object
/// pointer. The TLS index is read from the loader's four-byte global, while
/// the checker runtime supplies the fabricated slot value. The helper call is
/// scripted through the runtime; the caller's one stack argument is cleaned by
/// this function.
lf_checker_rt::export!(thiscall, rw_00D5B380(_this_ptr: u32, resource: u32) -> u32 {
    unsafe {
        const TLS_INDEX_GLOBAL_VA: u32 = 0x017A_BA14;
        const RESOURCE_INITIALIZER_FIELD: u32 = 0x1c;
        const REFERENCE_COUNT: u32 = 0x14;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let tls_index = lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL_VA).read_unaligned() as usize;
        let before_call_object = lf_checker_rt::tls_slot(tls_index);
        let before_call_count = read_u32(before_call_object.wrapping_add(REFERENCE_COUNT));
        write_u32(before_call_object.wrapping_add(REFERENCE_COUNT), before_call_count.wrapping_add(1));

        let initializer_argument = resource.wrapping_add(RESOURCE_INITIALIZER_FIELD);
        let _ = lf_checker_rt::callee_cdecl!(1, u32, initializer_argument);

        let after_call_object = lf_checker_rt::tls_slot(tls_index);
        let after_call_count = read_u32(after_call_object.wrapping_add(REFERENCE_COUNT));
        write_u32(after_call_object.wrapping_add(REFERENCE_COUNT), after_call_count.wrapping_sub(1));
        after_call_object
    }
});
