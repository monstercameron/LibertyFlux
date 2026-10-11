// original: 0x005DCCD0 initialize_task_event

/// Configure the event object for a pedestrian task and return it.
///
/// The initializer forwards the caller's context, event kind 0x19d, data
/// pointer 0xfe0d70 and fixed control words to the event constructor. It
/// passes the two f32 values +4.0 and -4.0 in that order, then installs the
/// task event vtable at offset zero and returns `this`. The constructor call
/// receives eight stack words under the game's 32-bit thiscall convention.
lf_checker_rt::export!(thiscall, rw_005dccd0(this: u32, context: u32) -> u32 {
    const EVENT_KIND: u32 = 0x019D;
    const EVENT_DATA: u32 = 0x00FE0D70;
    const CONTROL_FLAGS: u32 = 0;
    const EVENT_OPTIONS: u32 = 2;
    const LOWER_BOUND_F32_BITS: u32 = 0x40800000;
    const UPPER_BOUND_F32_BITS: u32 = 0xC0800000;
    const EVENT_VTABLE: u32 = 0x00EB73CC;

    let _ = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        CONTROL_FLAGS,
        EVENT_OPTIONS,
        LOWER_BOUND_F32_BITS,
        UPPER_BOUND_F32_BITS,
        context,
        EVENT_KIND,
        lf_checker_rt::relocated(EVENT_DATA),
        CONTROL_FLAGS,
    );
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(EVENT_VTABLE));
    }
    this
});
