// original: 0x006675c0 rage::ptxTimeLine::reset

/// Reset the timeline object and return its original `this` pointer. Store the
/// relocated timeline vtable at offset `+0`, then call the entry reset helper
/// with `this + 0x10`, the incoming argument, and the original `this` in that
/// stack order. If TLS slot one contains a service object, call its lookup
/// method with a pointer to the timeline value at `+0x18`. A signed result of
/// `-1` clears that value; otherwise, when the value is nonzero, call the
/// service adjustment method and add its result with 32-bit wrapping. This is
/// a thiscall method with one stack argument. Its three outgoing methods are
/// intercepted and their call order and arguments are compared.
lf_checker_rt::export!(thiscall, rw_006675c0(this: u32, parameter: u32) -> u32 {
    unsafe {
        const TIMELINE_VTABLE_FILE_VA: u32 = 0x00fe266c;
        const TIMELINE_VALUE_OFFSET: u32 = 0x18;
        const SERVICE_TLS_OFFSET: u32 = 4;
        const ENTRY_RESET_CALLEE: u32 = 1;
        const VALUE_LOOKUP_CALLEE: u32 = 2;
        const VALUE_ADJUST_CALLEE: u32 = 3;
        const SIGNED_FAILURE: u32 = u32::MAX;

        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        write_word(this, lf_checker_rt::relocated(TIMELINE_VTABLE_FILE_VA));
        lf_checker_rt::callee_thiscall!(ENTRY_RESET_CALLEE, u32, this + 0x10, parameter, this);

        let tls_base = lf_checker_rt::tls_slot(0);
        let service_object = read_word(tls_base + SERVICE_TLS_OFFSET);
        if service_object != 0 {
            let service_this = read_word(service_object);
            let timeline_value = this + TIMELINE_VALUE_OFFSET;
            let lookup_status = lf_checker_rt::callee_thiscall!(
                VALUE_LOOKUP_CALLEE,
                u32,
                service_this,
                timeline_value
            );

            if lookup_status == SIGNED_FAILURE {
                write_word(timeline_value, 0);
            } else {
                let value = read_word(timeline_value);
                if value != 0 {
                    let adjustment = lf_checker_rt::callee_thiscall!(
                        VALUE_ADJUST_CALLEE,
                        u32,
                        service_this,
                        value
                    );
                    write_word(timeline_value, value.wrapping_add(adjustment));
                }
            }
        }

        this
    }
});
