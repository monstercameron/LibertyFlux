// original: 0x00c0a990 stream_item_reset_and_forward (proposed)

/// Clear three fields of the streaming item, then tail-jump to the worker.
///
/// `this` points to the item's owner and `arg` to the item: the dwords at
/// `F0`, `F1` and `F2` past `arg` are zeroed, and control passes to the worker
/// (callee 1) with the owner pointer advanced by `OWNER_BIAS` and the same
/// `arg`. The worker's result is this function's result.
///
/// Original: 0x00c0a990 (thiscall, one stack word, ends in a tail jump).
lf_checker_rt::export!(thiscall, rw_00c0a990(this: u32, arg: u32) -> u32 {
    unsafe {
        const F0: u32 = 0x08;
        const F1: u32 = 0x0c;
        const F2: u32 = 0x10;
        const OWNER_BIAS: u32 = 0x3008;
        const WORKER: u32 = 1;
        (arg.wrapping_add(F0) as *mut u32).write_unaligned(0);
        (arg.wrapping_add(F1) as *mut u32).write_unaligned(0);
        (arg.wrapping_add(F2) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(WORKER, u32, this.wrapping_add(OWNER_BIAS), arg)
    }
});
