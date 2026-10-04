// original: 0x00d28cd0 target_slot_float_get (proposed)

/// Copy a found slot record's float to an output word and test it for zero.
///
/// Looks up `key` with the slot-record finder (intercepted); a miss returns
/// 0 and writes nothing. On a hit, copies the record's float at `+0x18` to
/// `*out` and returns 1 when it is nonzero, else 0. Any NaN counts as
/// nonzero (the original's unordered-test-then-jump-not-parity path).
///
/// Original: 0x00D28CD0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d28cd0(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x18;
        let rec: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, key);
        if rec == 0 {
            return 0;
        }
        let v = unsafe { ((rec + FIELD_OFF) as *const u32).read_unaligned() };
        unsafe { (out as *mut u32).write_unaligned(v) };
        if f32::from_bits(v) != 0.0 { 1 } else { 0 }
    }
});
