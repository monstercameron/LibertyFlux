// original: 0x00CC8720 fragInstGta::vf64

/// Stores the receiver pointer in the supplied object's field at `+0x0c`,
/// then calls the object's setup helper with the object as its thiscall
/// receiver and constants `0x80` and `1`. The helper's EAX value is returned
/// unchanged, and the method removes its one stack argument.
lf_checker_rt::export!(thiscall, rw_00cc8720(this: u32, object: u32) -> u32 {
    const RECEIVER_FIELD: u32 = 0x0c;
    const SETUP: u32 = 1;
    const SETUP_KIND: u32 = 0x80;
    const SETUP_FLAGS: u32 = 1;

    unsafe {
        (object.wrapping_add(RECEIVER_FIELD) as *mut u32).write_unaligned(this);
    }
    lf_checker_rt::callee_thiscall!(SETUP, u32, object, SETUP_KIND, SETUP_FLAGS)
});
