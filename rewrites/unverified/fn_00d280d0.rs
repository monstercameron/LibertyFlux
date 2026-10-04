// original: 0x00d280d0 target_slot_copy_if_live (proposed)

/// Copy a found slot record's header to an output buffer and test liveness.
///
/// When `key` is null, the default key comes from the key helper
/// (intercepted) instead. Looks the key up with the slot-record finder
/// (intercepted); a miss returns 1 and writes nothing. On a hit, copies the
/// record's words at `+0x00` through `+0x0c` to `*out` when `out` is nonzero,
/// then returns 1 if the record's float at `+0x24` is strictly above 0.0,
/// else 0 (NaN yields 0).
///
/// Original: 0x00D280D0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d280d0(this: u32, out: u32, key: u32) -> u32 {
    unsafe {
        const LIVE_OFF: u32 = 0x24;
        let k = if key == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, this)
        } else {
            key
        };
        let rec: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, k);
        if rec == 0 {
            return 1;
        }
        if out != 0 {
            for off in [0u32, 4, 8, 12] {
                let v = unsafe { ((rec + off) as *const u32).read_unaligned() };
                unsafe { ((out + off) as *mut u32).write_unaligned(v) };
            }
        }
        let live = unsafe { f32::from_bits(((rec + LIVE_OFF) as *const u32).read_unaligned()) };
        if live > 0.0 { 1 } else { 0 }
    }
});
