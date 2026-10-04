// original: 0x00967640 forward_notify_return_this
/// Run the 0x968540 helper on `this` and return `this` unchanged.
///
/// Passes `this` through in ECX (thiscall, no stack arguments); the
/// helper's answer is ignored and the pointer itself is the result.
///
/// Original: 0x00967640 (thiscall, no stack words).

export!(thiscall, rw_00967640(this: u32) -> u32 {
    callee_thiscall!(1, u32, this);
    this
});
