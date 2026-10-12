// original: 0x00601000 zero_8_bytes

/// Zero the first two words of the object at `this` and return `this`.
///
/// The original stores zero to `[this + 4]` first and then to `[this]`;
/// only the final memory is observable, so the order is irrelevant.
/// Thiscall: the object address arrives in ECX, no stack arguments, and
/// the same address is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00601000(this: u32) -> u32 {
    unsafe {
        (this as *mut u32).write(0);
        ((this + 4) as *mut u32).write(0);
        this
    }
});
