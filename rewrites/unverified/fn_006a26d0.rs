// original: 0x006A26D0 initialize_stream_record
/// Runs the base-record initializer, installs this record's relocated vtable,
/// and updates the value at byte offset twelve through the current thread's
/// first TLS context.
///
/// A null context or zero value clears the field. Otherwise the validator is
/// called with the context's first pointer and the field address. Its
/// unsigned `u32::MAX` result is the invalid sentinel; other results call the
/// updater, whose return is added with 32-bit wrapping arithmetic. A nonzero
/// sum is sent to the notification helper. The record cleanup helper always
/// receives `this + 0x40`; when the value at offset eight is nonzero, the
/// optional cleanup and finalizer run in that order. Calls are intercepted by
/// the contract, and the function returns `this` after consuming one
/// reserved stack word.
const OBJECT_VTABLE_26D0: u32 = 0x00FE_3F7C;
const TLS_CONTEXT_SLOT_26D0: usize = 0;
const VALUE_WORD_26D0: usize = 3;
const OPTIONAL_VALUE_WORD_26D0: usize = 2;
const BASE_MEMBER_OFFSET_26D0: u32 = 0x40;
const OPTIONAL_MEMBER_OFFSET_26D0: u32 = 0x10;
const INVALID_VALUE_26D0: u32 = u32::MAX;

lf_checker_rt::export!(thiscall, rw_006a26d0(this: u32, _reserved: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this, this);
    unsafe {
        core::ptr::write_volatile(
            this as *mut u32,
            lf_checker_rt::relocated(OBJECT_VTABLE_26D0),
        );
    }
    let tls_block = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT_26D0) as *const u32;
    let tls_context = unsafe {
        core::ptr::read_volatile(tls_block.add(TLS_CONTEXT_SLOT_26D0 + 1))
    };
    let fields = this as *mut u32;
    let value_slot = unsafe { fields.add(VALUE_WORD_26D0) };
    let current_value = unsafe { core::ptr::read_volatile(value_slot) };
    if tls_context == 0 || current_value == 0 {
        unsafe { core::ptr::write_volatile(value_slot, 0) };
    } else {
        let validator_client = unsafe { core::ptr::read_volatile(tls_context as *const u32) };
        let validation = lf_checker_rt::callee_thiscall!(
            2,
            u32,
            validator_client,
            value_slot as u32,
        );
        if validation == INVALID_VALUE_26D0 {
            unsafe { core::ptr::write_volatile(value_slot, 0) };
        } else {
            let current_value = unsafe { core::ptr::read_volatile(value_slot) };
            let increment = lf_checker_rt::callee_thiscall!(3, u32, tls_context, current_value);
            let updated_value = current_value.wrapping_add(increment);
            unsafe { core::ptr::write_volatile(value_slot, updated_value) };
            if updated_value != 0 {
                let _ = lf_checker_rt::callee_thiscall!(4, u32, updated_value, tls_context);
            }
        }
    }
    let _ = lf_checker_rt::callee_thiscall!(5, u32, this + BASE_MEMBER_OFFSET_26D0);
    let optional_value = unsafe {
        core::ptr::read_volatile(fields.add(OPTIONAL_VALUE_WORD_26D0))
    };
    if optional_value != 0 {
        let _ = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            this + OPTIONAL_MEMBER_OFFSET_26D0,
            optional_value,
        );
        let _ = lf_checker_rt::callee_thiscall!(7, u32, this);
    }
    this
});
