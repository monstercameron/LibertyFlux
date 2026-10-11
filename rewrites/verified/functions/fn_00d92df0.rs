// original: 0x00D92DF0 mainloop_timing_request_halfword_view
lf_checker_rt::export!(thiscall, rw_00d92df0(array_owner: u32, element_index: u32) -> u32 {
    const TLS_INDEX_GLOBAL: u32 = 0x017ABA14;
    const SERVICE_POINTER_OFFSET: u32 = 0x10;
    const ELEMENT_STRIDE: u32 = 2;

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
