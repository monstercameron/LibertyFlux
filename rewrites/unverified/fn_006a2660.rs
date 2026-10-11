// original: 0x006A2660 update_stream_object
/// Writes the object's relocated vtable, then updates its value at byte
/// offset eight using the current thread's first TLS context.
///
/// The TLS slot points to a block whose second word is the context. A null
/// context or a zero object value clears that field. Otherwise the validator
/// is called with the context's first pointer and the address of the field.
/// Its unsigned result equal to `u32::MAX` is the invalid sentinel and clears
/// the field. For every other result, the updater's return value is added to
/// the current field with 32-bit wrapping arithmetic. A nonzero updated
/// value is passed with the context to the notification helper. The base
/// constructor, validator, updater and notification remain outgoing calls;
/// their scripts are part of the contract. The function returns `this` and
/// consumes one reserved stack word.
const OBJECT_VTABLE_2660: u32 = 0x00FE_3F34;
const TLS_CONTEXT_SLOT_2660: usize = 0;
const OBJECT_VALUE_WORD_2660: usize = 2;
const INVALID_VALUE_2660: u32 = u32::MAX;

lf_checker_rt::export!(thiscall, rw_006a2660(this: u32, _reserved: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        core::ptr::write_volatile(
            this as *mut u32,
            lf_checker_rt::relocated(OBJECT_VTABLE_2660),
        );
    }
    let tls_block = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT_2660) as *const u32;
    let tls_context = unsafe {
        core::ptr::read_volatile(tls_block.add(TLS_CONTEXT_SLOT_2660 + 1))
    };
    let object_words = this as *mut u32;
    let value_slot = unsafe { object_words.add(OBJECT_VALUE_WORD_2660) };
    let current_value = unsafe { core::ptr::read_volatile(value_slot) };
    if tls_context == 0 || current_value == 0 {
        unsafe { core::ptr::write_volatile(value_slot, 0) };
        return this;
    }
    let validator_client = unsafe { core::ptr::read_volatile(tls_context as *const u32) };
    let validation = lf_checker_rt::callee_thiscall!(
        2,
        u32,
        validator_client,
        value_slot as u32,
    );
    if validation == INVALID_VALUE_2660 {
        unsafe { core::ptr::write_volatile(value_slot, 0) };
        return this;
    }
    let increment = lf_checker_rt::callee_thiscall!(3, u32, tls_context, current_value);
    let updated_value = current_value.wrapping_add(increment);
    unsafe { core::ptr::write_volatile(value_slot, updated_value) };
    if updated_value != 0 {
        let _ = lf_checker_rt::callee_thiscall!(4, u32, updated_value, tls_context);
    }
    this
});
