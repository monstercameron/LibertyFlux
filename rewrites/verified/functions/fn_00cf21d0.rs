// original: 0x00CF21D0 playstat_stamp_elapsed

/// Store the current session elapsed-time value into a play-stat record.
///
/// `this` is passed to the intercepted elapsed-time helper as its thiscall
/// object pointer. The helper's EAX result is written to the record at `+0x0c`
/// and is also returned unchanged. No other record field is modified.
lf_checker_rt::export!(thiscall, rw_fn_00cf21d0(this: u32) -> u32 {
    const ELAPSED_FIELD: usize = 0x0c;
    const ELAPSED_CALLEE: u32 = 1;
    let elapsed: u32 = lf_checker_rt::callee_thiscall!(ELAPSED_CALLEE, u32, this);
    unsafe { (this as *mut u8).add(ELAPSED_FIELD).cast::<u32>().write_unaligned(elapsed); }
    elapsed
});
