// original: 0x00cc5f90 queue_push_back
/// Append `val` to the tail of the small intrusive queue at `this`.
///
/// The old tail's link is forwarded, the count at `+8` increments, and an
/// empty queue's head is set too. Returns the appended value.
export!(thiscall, rw_00cc5f90(this: *mut u8, val: u32) -> u32 {
    unsafe {
        let old_tail = *(this.add(4) as *const u32);
        *(this.add(4) as *mut u32) = val;
        if old_tail != 0 {
            *(((old_tail) as *mut u8).add(4) as *mut u32) = val;
        }
        let count = *(this.add(8) as *const u32);
        *(this.add(8) as *mut u32) = count.wrapping_add(1);
        if *(this as *const u32) == 0 {
            *(this as *mut u32) = val;
        }
        val
    }
});
