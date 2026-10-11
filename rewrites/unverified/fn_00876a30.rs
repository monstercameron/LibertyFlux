// original: 0x00876A30 rage::crmtRequestExtrapolate::vf2


/// Allocate an extrapolation result from the first incoming stack word,
/// initialize it from this request's start/end values, two mode bytes and
/// XMM1 f32 weight, and conditionally update this request when +0x14 is
/// nonzero. The method is thiscall with two stack words and returns the
/// allocation pointer. The initializer's f32 weight is passed in XMM1.
lf_checker_rt::export!(thiscall, rw_00876a30(this: u32, first_argument: u32, second_argument: u32) -> u32 {
    const ELAPSED_TICKS: u32 = 0x14;
    const WEIGHT: u32 = 0x18;
    const START_VALUE: u32 = 0x1c;
    const END_VALUE: u32 = 0x20;
    const MODE_A: u32 = 0x24;
    const MODE_B: u32 = 0x25;
    const ALLOCATE: u32 = 1;
    const INITIALIZE: u32 = 2;
    const UPDATE: u32 = 3;

    unsafe {
        let allocated = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, first_argument);
        let start_value = ((this + START_VALUE) as *const u32).read_unaligned();
        let end_value = ((this + END_VALUE) as *const u32).read_unaligned();
        let mode_a = u32::from(((this + MODE_A) as *const u8).read());
        let mode_b = u32::from(((this + MODE_B) as *const u8).read());
        let weight = ((this + WEIGHT) as *const u32).read_unaligned();
        let _ = lf_checker_rt::callee_thiscall!(
            INITIALIZE, u32, allocated, start_value, end_value, mode_a, mode_b, weight,
        );

    let elapsed_ticks = ((this + ELAPSED_TICKS) as *const u32).read_unaligned();
    if elapsed_ticks != 0 {
        let _ = lf_checker_rt::callee_thiscall!(UPDATE, u32, this, first_argument,
                                                 second_argument, allocated);
    }
    allocated
    }
});
