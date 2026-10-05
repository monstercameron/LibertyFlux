// original: 0x0093e180 ptr_insertion_step (proposed)

/// Insert `val` into the sorted pointer run ending at `pos`.
///
/// The run holds pointers to objects whose first word is the sort key.
/// Elements strictly greater than `val`'s key shift one slot right until
/// the first element not above it; `val` is stored there. Returns the slot
/// probed last (`pos - 4` per shift plus one), matching the original's
/// register residue. The caller's third word is never read.
///
/// Original: 0x0093e180 (cdecl, two stack words read).
lf_checker_rt::export!(cdecl, rw_0093e180(pos: u32, val: u32) -> u32 {
    const OBJ_KEY: u32 = 0;
    unsafe {
        let val_key = ((val + OBJ_KEY) as *const u32).read_unaligned();
        let mut slot = pos;
        let mut probe = pos.wrapping_sub(4);
        let mut prev = (probe as *const u32).read_unaligned();
        if val_key >= ((prev + OBJ_KEY) as *const u32).read_unaligned() {
            (slot as *mut u32).write_unaligned(val);
            return probe;
        }
        loop {
            (slot as *mut u32).write_unaligned(prev);
            slot = probe;
            probe = probe.wrapping_sub(4);
            prev = (probe as *const u32).read_unaligned();
            if val_key >= ((prev + OBJ_KEY) as *const u32).read_unaligned() {
                (slot as *mut u32).write_unaligned(val);
                return probe;
            }
        }
    }
});
