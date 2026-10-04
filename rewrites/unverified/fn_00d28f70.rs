// original: 0x00d28f70 target_slot_header_copy (proposed)

/// Copy the four-word header of a found slot record to an output buffer.
///
/// Looks up `key` with the slot-record finder (intercepted). When a record is
/// found, copies its words at `+0x00` through `+0x0c` to `*out` and returns 1;
/// otherwise returns 0 and writes nothing.
///
/// Original: 0x00D28F70 (thiscall, two stack arguments).
lf_checker_rt::export!(thiscall, rw_00d28f70(this: u32, key: u32, out: u32) -> u32 {
    unsafe {
        let rec: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, key);
        if rec == 0 {
            return 0;
        }
        for off in [0u32, 4, 8, 12] {
            let v = unsafe { ((rec + off) as *const u32).read_unaligned() };
            unsafe { ((out + off) as *mut u32).write_unaligned(v) };
        }
        1
    }
});
