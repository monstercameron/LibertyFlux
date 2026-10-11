// original: 0x00668190 rage::ptxEventEmitter::vf1

/// Returns one when the event record's dword at +0x104 has bit 2 clear, and
/// zero when that bit is set. A null event-record pointer returns zero. The
/// `this` pointer in ECX is unused; the one stack argument is callee-cleaned.
lf_checker_rt::export!(thiscall, rw_00668190(_this: u32, event_record: u32) -> u32 {
    const EVENT_FLAGS: u32 = 0x104;
    const REVERSED_BIT: u32 = 2;

    if event_record == 0 {
        return 0;
    }

    let flags = unsafe {
        (event_record.wrapping_add(EVENT_FLAGS) as *const u32).read_unaligned()
    };
    1 ^ ((flags >> REVERSED_BIT) & 1)
});
