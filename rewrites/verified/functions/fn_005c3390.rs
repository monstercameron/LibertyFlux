// original: 0x005C3390 ped_list_cleanup_and_tls_service

/// Walks `count` entries in a strided object list and calls each non-null
/// object's virtual slot 0 with the value 1. If the list pointer is non-null,
/// it then obtains a thread-local service object from TLS slot 0 and calls its
/// virtual slot 3 with the list pointer. The list entries begin at offset
/// 0x154 and are 0x168 bytes apart. The two incoming arguments are stdcall
/// stack words; the final call's EAX result is returned.
lf_checker_rt::export!(stdcall, rw_005c3390(elements: u32, count: u32) -> u32 {
    unsafe {
        const LIST_START: u32 = 0x154;
        const ELEMENT_STRIDE: u32 = 0x168;
        const VTABLE_POINTER: u32 = 0x00;
        const SLOT_FIRST: u32 = 0x00;
        const SLOT_TLS_SERVICE: u32 = 0x0c;
        const ITEM_ARGUMENT: u32 = 1;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn call_thiscall(target: u32, this: u32, argument: u32) -> u32 {
            let method: extern "thiscall" fn(u32, u32) -> u32 =
                unsafe { core::mem::transmute(target as usize) };
            method(this, argument)
        }

        let signed_count = count as i32;
        let mut index = 0u32;
        while (index as i32) < signed_count {
            let entry = read_word(elements + LIST_START + index * ELEMENT_STRIDE);
            if entry != 0 {
                let vtable = read_word(entry + VTABLE_POINTER);
                let target = read_word(vtable + SLOT_FIRST);
                let _ = call_thiscall(target, entry, ITEM_ARGUMENT);
            }
            index += 1;
        }

        if elements == 0 {
            return 0;
        }
        let tls_context = lf_checker_rt::tls_slot(0);
        let service = read_word(tls_context + 8);
        let service_vtable = read_word(service + VTABLE_POINTER);
        let service_target = read_word(service_vtable + SLOT_TLS_SERVICE);
        call_thiscall(service_target, service, elements)
    }
});
