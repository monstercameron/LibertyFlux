// original: 0x00C78330 phbound_tls_scope_update (proposed)

/// Temporarily link a four-word exception record through TLS slot
/// `*0x017aba14`, increment the scope counter at `0x018b7a50`, and read a
/// payload pointer from `context+8`. If the payload is non-null, call the
/// helper with the payload in ECX and a pointer to the record as its one
/// stack argument.
/// Finally restore the TLS link, decrement the counter, and return the
/// payload pointer. The record fields are the context pointer, previous TLS
/// link, handler constant, and active flag.
lf_checker_rt::export!(cdecl, rw_phbound_tls_scope_update(_unused: u32, context: u32) -> u32 {
    unsafe {
        const TLS_INDEX_VA: u32 = 0x017a_ba14;
        const SCOPE_COUNT_VA: u32 = 0x018b_7a50;
        const TLS_LINK: u32 = 4;
        const CONTEXT_PAYLOAD: u32 = 8;
        const EXCEPTION_HANDLER: u32 = 0x00ed_4224;
        const HELPER: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let tls_index = unsafe { lf_checker_rt::global::<u32>(TLS_INDEX_VA).read_volatile() } as usize;
        let tls_record = lf_checker_rt::tls_slot(tls_index);
        let previous = unsafe { read_u32(tls_record + TLS_LINK) };
        let mut record_storage = core::mem::MaybeUninit::<[u8; 16]>::uninit();
        let record_bytes = record_storage.as_mut_ptr().cast::<u8>();
        let record_pointer = record_bytes as u32;
        let handler = lf_checker_rt::relocated(EXCEPTION_HANDLER);
        unsafe {
            record_bytes.cast::<u32>().write_unaligned(context);
            record_bytes.add(4).cast::<u32>().write_unaligned(previous);
            record_bytes.add(8).cast::<u32>().write_unaligned(handler);
            record_bytes.add(12).write(1);
        }
        unsafe { write_u32(tls_record + TLS_LINK, record_pointer) };

        let scope_count = lf_checker_rt::global::<u32>(SCOPE_COUNT_VA);
        let old_count = unsafe { scope_count.read_volatile() };
        unsafe { scope_count.write_volatile(old_count.wrapping_add(1)) };

        let payload = unsafe { read_u32(context + CONTEXT_PAYLOAD) };
        if payload != 0 {
            let _ = lf_checker_rt::callee_thiscall!(HELPER, u32, payload, record_pointer);
        }

        unsafe { write_u32(tls_record + TLS_LINK, previous) };
        let current_count = unsafe { scope_count.read_volatile() };
        unsafe { scope_count.write_volatile(current_count.wrapping_sub(1)) };
        payload
    }
});
