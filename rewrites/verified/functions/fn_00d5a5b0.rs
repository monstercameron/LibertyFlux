// original: 0x00d5a5b0 ccam_market_dtor
/// Destructor for CCamMarket: restores this class's virtual table, runs the
/// member teardown, then tail-jumps to the base destructor.
///
/// Writes the relocated address of the class table `VTABLE` at `[this]`,
/// calls the member teardown (intercepted callee 1, thiscall, no stack
/// arguments), then forwards `this` to the base destructor (intercepted
/// tail callee 2, thiscall, no stack arguments) and returns its result.
///
/// Original: thiscall, no stack arguments; ends in a tail jump whose result
/// is the function's result.
lf_checker_rt::export!(thiscall, rw_00d5a5b0 (this: u32) -> u32 {
    unsafe {
        const TEARDOWN: u32 = 1;
        const BASE_DTOR: u32 = 2;
        const VTABLE: u32 = 0x00EE7D80;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _: u32 = lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
