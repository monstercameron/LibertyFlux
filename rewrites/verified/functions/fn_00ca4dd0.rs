// original: 0x00ca4dd0 CEventHandler::initialize_default_events

/// Registers the default event set through one intercepted thiscall, then
/// installs the event-handler vtable and returns the object pointer. The
/// registration receives the fixed count, table pointer, flags and float
/// defaults observed at this call site. The object layout is 32-bit.
lf_checker_rt::export!(thiscall, rw_00ca4dd0(this: u32) -> u32 {
    const EVENT_COUNT: u32 = 0x019b;
    const EVENT_TABLE_VA: u32 = 0x00ed_7b48;
    const ONE_BITS: u32 = 0x3f80_0000;
    const FOUR_BITS: u32 = 0x4080_0000;
    const EVENT_HANDLER_VTABLE_VA: u32 = 0x00ed_7af4;

    let _ = lf_checker_rt::callee_thiscall!(
        1,
        u32,
        this,
        0,
        1,
        FOUR_BITS,
        EVENT_COUNT,
        lf_checker_rt::relocated(EVENT_TABLE_VA),
        0,
        ONE_BITS,
        0
    );
    unsafe {
        (this as *mut u32).write_unaligned(
            lf_checker_rt::relocated(EVENT_HANDLER_VTABLE_VA),
        );
    }
    this
});
