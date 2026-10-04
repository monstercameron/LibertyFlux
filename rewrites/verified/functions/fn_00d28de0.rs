// original: 0x00d28de0 target_slot_field_get (proposed)

/// Copy one field of a found slot record to an output word.
///
/// Looks up `key` with the slot-record finder (intercepted). When a record is
/// found, stores its word at `+0x10` into `*out` and returns 1; otherwise
/// returns 0 and writes nothing.
///
/// Original: 0x00D28DE0 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d28de0(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        const FIELD_OFF: u32 = 0x10;
        let rec: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, key);
        if rec == 0 {
            return 0;
        }
        let v = unsafe { ((rec + FIELD_OFF) as *const u32).read_unaligned() };
        unsafe { (out as *mut u32).write_unaligned(v) };
        1
    }
});
