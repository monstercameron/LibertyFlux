// original: 0x006A2760 initialize_configured_stream_record
/// Runs the base initializer, stores the second stack argument at byte offset
/// eight, and installs the relocated record vtable. It then performs the TLS
/// validation/update sequence on the value at offset twelve, using unsigned
/// `u32::MAX` as the invalid sentinel and 32-bit wrapping addition for the
/// updater result.
///
/// A nonzero updated value is passed with the TLS context to the notification
/// helper. The record cleanup helper always receives `this + 0x40`. If the
/// stored second argument is nonzero, the optional cleanup receives
/// `this + 0x10` and that value, followed by a finalizer call. The first stack
/// argument is unused. All outgoing helpers are intercepted and compared by
/// the contract; the function returns `this` and consumes both stack words.
const OBJECT_VTABLE_2760: u32 = 0x00FE_3F7C;
const TLS_CONTEXT_SLOT_2760: usize = 0;
const VALUE_WORD_2760: usize = 3;
const OPTIONAL_VALUE_WORD_2760: usize = 2;
const BASE_MEMBER_OFFSET_2760: u32 = 0x40;
const OPTIONAL_MEMBER_OFFSET_2760: u32 = 0x10;
const INVALID_VALUE_2760: u32 = u32::MAX;

lf_checker_rt::export!(thiscall, rw_006a2760(this: u32, _first_arg: u32, second_arg: u32) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe {
        core::ptr::write_volatile(
            this as *mut u32,
            lf_checker_rt::relocated(OBJECT_VTABLE_2760),
        );
    }
    let fields = this as *mut u32;
    unsafe { core::ptr::write_volatile(fields.add(OPTIONAL_VALUE_WORD_2760), second_arg) };
    let tls_block = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT_2760) as *const u32;
    let tls_context = unsafe { core::ptr::read_volatile(tls_block.add(1)) };
    let value_slot = unsafe { fields.add(VALUE_WORD_2760) };
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
        if validation == INVALID_VALUE_2760 {
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
    let _ = lf_checker_rt::callee_thiscall!(5, u32, this + BASE_MEMBER_OFFSET_2760);
    let optional_value = unsafe {
        core::ptr::read_volatile(fields.add(OPTIONAL_VALUE_WORD_2760))
    };
    if optional_value != 0 {
        let _ = lf_checker_rt::callee_thiscall!(
            6,
            u32,
            this + OPTIONAL_MEMBER_OFFSET_2760,
            optional_value,
        );
        let _ = lf_checker_rt::callee_thiscall!(7, u32, this);
    }
    this
});
