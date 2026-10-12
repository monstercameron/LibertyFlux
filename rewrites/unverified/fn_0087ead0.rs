// original: 0x0087EAD0 unnamed_0087ead0

/// Initialize the parameterized form of a buffer state.
///
/// `this` addresses a 28-byte state. Initialization first clears its words,
/// sets the capacity at `+0x14` to 256 and the marker at `+0x18` to `u32::MAX`,
/// then calls the no-argument helper twice. It stores the first stack argument
/// at `+0x00`, sets `+0x04` to `0x101`, `+0x0c` to six and `+0x10` to zero,
/// and returns `this`. The second stack argument is unused. Helper calls are
/// intercepted by the checker.
///
/// Original: thiscall with two stack words and callee cleanup.
lf_checker_rt::export!(thiscall, rw_0087ead0(
    this: u32,
    initial_value: u32,
    unused_argument: u32
) -> u32 {
    const VALUE_OFFSET: u32 = 0;
    const TYPE_OFFSET: u32 = 0x04;
    const COUNTER_OFFSET: u32 = 0x0C;
    const POSITION_OFFSET: u32 = 0x10;
    const CAPACITY_OFFSET: u32 = 0x14;
    const LIMIT: u32 = 0x100;
    const TYPE_TAG: u32 = 0x101;
    const INVALID_INDEX: u32 = u32::MAX;
    const INITIALIZE_HELPER: u32 = 1;

    let _ = unused_argument;
    unsafe {
        for offset in [VALUE_OFFSET, TYPE_OFFSET, 8, COUNTER_OFFSET, POSITION_OFFSET] {
            (this.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this.wrapping_add(CAPACITY_OFFSET) as *mut u32).write_unaligned(LIMIT);
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(INVALID_INDEX);
        let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_HELPER, u32, this);
        let _ = lf_checker_rt::callee_thiscall!(INITIALIZE_HELPER, u32, this);
        (this.wrapping_add(VALUE_OFFSET) as *mut u32).write_unaligned(initial_value);
        (this.wrapping_add(TYPE_OFFSET) as *mut u32).write_unaligned(TYPE_TAG);
        (this.wrapping_add(COUNTER_OFFSET) as *mut u32).write_unaligned(6);
        (this.wrapping_add(POSITION_OFFSET) as *mut u32).write_unaligned(0);
    }
    this
});
