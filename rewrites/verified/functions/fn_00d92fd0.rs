// original: 0x00D92FD0 mainloop_timing_request_seventeen_byte_view

/// Load the timer-array base from the object and request one indexed view
/// through the TLS-selected service object's virtual slot at byte offset eight.
/// The service receives `base + index * 16` with 32-bit wrapping, the base, and
/// one; its 32-bit return value is passed through. The contract fixes the TLS
/// index to one of the fabricated slots and compares the service call and result.
lf_checker_rt::export!(thiscall, rw_00d92fd0(array_owner: u32, element_index: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x017ABA14;
    const SERVICE_POINTER_OFFSET: u32 = 0x10;
    const ELEMENT_STRIDE: u32 = 16;

    let array_base = unsafe { read_timing_word(array_owner) };
    let tls_index = unsafe {
        lf_checker_rt::global::<u32>(TLS_INDEX_GLOBAL).read_unaligned() as usize
    };
    let tls_root = lf_checker_rt::tls_slot(tls_index);
    let service_object = unsafe {
        read_timing_word(tls_root.wrapping_add(SERVICE_POINTER_OFFSET))
    };
    let requested_address = array_base.wrapping_add(element_index.wrapping_mul(ELEMENT_STRIDE));
    unsafe { invoke_timing_slot_8(service_object, requested_address, array_base, 1) }
});
